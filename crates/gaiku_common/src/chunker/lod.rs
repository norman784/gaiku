use super::common::*;
use crate::{
  atlas::AtlasifyMut,
  boundary::Boundary,
  boxify::Boxify,
  chunk::{Chunkify, ChunkifyMut},
  interpolators::*,
};
use glam::Vec3;

///
/// This chunker will create a LOD tree
///
/// This is done using an octree
/// with greater detail at the deeper
/// leaves.
///
/// In order to calculate the sample values
/// at arbitary resolution trilinear interpolation
/// is used.
///
pub struct LodChunker<C> {
  /// The interpolator used to calculate the arbitary points
  interpolator: TriLinear,
  /// The tree that holds the data
  tree: OctTree<Chunked<C>>,
  /// The point we calculate the lod distances from
  observation_point: Vec3,
  /// The size in voxels of every generated chunk
  chunk_sizes: [u16; 3],
  /// The lod distance.
  /// The hightest detail lod is distance from center of the leaf
  /// is less than this distance
  /// The second highest lod is twice this distance
  /// The third highest lod is 4x this distance
  /// The forth highest lod is 8x this distance etc
  lod_distance: f32,
}

impl<C> Chunker<C, f32, u8> for LodChunker<C>
where
  C: Chunkify<f32> + ChunkifyMut<f32> + AtlasifyMut<u8> + Boxify,
{
  ///
  /// Chunks the data with atlas values
  ///
  /// The data needs to have the length equal to the
  /// width * height * depth. But the atlas data does
  /// not any missing values will likely be replaced with
  /// defaults (usually zero)
  ///
  /// # Parameters
  ///
  /// * `data` - The source data for the values
  ///
  /// * `atlas_data` - The source data for the atlas values
  ///
  /// * `width` - width of the data
  ///
  /// * `height` - height of the data
  ///
  /// * `depth` - depth of the data
  ///
  /// # Returns
  ///
  /// returns description
  ///
  fn from_array_with_atlas(
    data: &[f32],
    atlas_data: &[u8],
    width: usize,
    height: usize,
    depth: usize,
  ) -> Self {
    Self {
      interpolator: TriLinear::from_array_with_atlas(data, atlas_data, width, height, depth),
      observation_point: [0., 0., 0.].into(),
      tree: OctTree::new(
        Boundary::new(
          &[0., 0., 0.].into(),
          &[width as f32, height as f32, depth as f32].into(),
        ),
        0,
      ),
      chunk_sizes: [16, 16, 16],
      lod_distance: 16.,
    }
  }

  ///
  /// Generates the chunks from the source data
  ///
  /// The source data needs to be already setup
  /// and ready before making this call
  ///
  /// This method is lazy and will only generate chunks
  /// visible from the `observation_point`
  ///
  fn generate_chunks(&mut self) {
    let root_size = self.tree.get_size();
    for leaf in
      OctTreeVisibleIterMut::new(&mut self.tree, self.observation_point, self.lod_distance)
    {
      if leaf.get_data().is_none() {
        // Need to update this leaf
        let location = leaf.get_origin();
        let scale = leaf.get_size() / root_size;
        let mut chunk = C::new(
          location.into(),
          self.chunk_sizes[0],
          self.chunk_sizes[1],
          self.chunk_sizes[2],
        );
        let chunk_sizes_f32: Vec3 = [
          self.chunk_sizes[0] as f32,
          self.chunk_sizes[1] as f32,
          self.chunk_sizes[2] as f32,
        ]
        .into();
        let delta = leaf.get_size() / (chunk_sizes_f32 - Vec3::one());
        for i in 0..self.chunk_sizes[0] as usize {
          for j in 0..self.chunk_sizes[1] as usize {
            for k in 0..self.chunk_sizes[2] as usize {
              let ijk: Vec3 = [i as f32, j as f32, k as f32].into();
              let d_pos: Vec3 = delta * ijk;
              let p_pos: Vec3 = location + d_pos;
              if let Ok(value) = self.interpolator.get_value(p_pos.into()) {
                chunk.set(i, j, k, value);
              }
              if let Ok(Some(atlas_value)) = self.interpolator.get_atlas_value(p_pos.into()) {
                chunk.set_atlas(i, j, k, atlas_value);
              }
            }
          }
        }

        let data = Chunked {
          location: location.into(),
          scale: scale.into(),
          chunk,
        };
        leaf.set_data(data);
      }
    }
  }

  ///
  /// Gets the resulting chunks
  ///
  /// This should be called after `generate_chunks`
  /// and only returns the visible chunks that have
  /// been generated
  ///
  /// # Returns
  ///
  /// returns a `Vec<& Chunked<C>>`
  ///
  fn get_chunks(&self) -> Vec<&Chunked<C>> {
    self
      .visible_iter()
      .map(|leaf| leaf.get_data())
      .flatten()
      .collect()
  }

  ///
  /// Gets the resulting chunks mutably
  ///
  /// This should be called after `generate_chunks`
  /// and only returns the visible chunks that have
  /// been generated
  ///
  /// # Returns
  ///
  /// returns a `Vec<&mut Chunked<C>>`
  ///
  fn get_chunks_mut(&mut self) -> Vec<&mut Chunked<C>> {
    self
      .visible_iter_mut()
      .map(|leaf| leaf.get_data_mut())
      .flatten()
      .collect()
  }
}

impl<C> LodChunker<C>
where
  C: Chunkify<f32> + ChunkifyMut<f32> + AtlasifyMut<u8> + Boxify,
{
  ///
  /// Sets the number of LODs
  ///
  /// A LOD tree starts with zero lods. If this is not called
  /// then only the lowest resolution will be avaliable
  ///
  /// # Parameters
  ///
  /// * `levels` - The number of LOD levels to use
  ///
  pub fn set_lod_levels(&mut self, levels: usize) {
    self.tree.resize_tree(levels);
  }

  ///
  /// Sets the distance that governs when the LOD will change
  ///
  /// The chosen LOD level (n) is calcaulted from
  /// the distance (d) of the `observation_point` and `lod_distance` (l)
  /// n = ⎣ln(d/l)/ln(2)⎦
  ///
  /// This should equate to this:
  /// The hightest detail lod less than `lod_distance` from center of the leaf
  /// The second highest lod is twice this distance
  /// The third highest lod is 4x this distance
  /// The forth highest lod is 8x this distance etc
  ///
  /// # Parameters
  ///
  /// * `lod_distance` - The distance to use when increasing the LOD
  ///
  pub fn set_lod_distance(&mut self, lod_distance: f32) {
    self.lod_distance = lod_distance;
  }

  ///
  /// This will change the chunk size that each LOD level has
  ///
  /// This will only change new chunks made after this point.
  /// If you want to regenerate all chunks first call `reset_chunks`
  ///
  /// # Parameters
  ///
  /// * `size` - size
  ///
  pub fn set_chunk_size(&mut self, size: [u16; 3]) {
    self.chunk_sizes = size;
  }

  ///
  /// This will clear all chunk data on all LODs
  ///
  pub fn reset_chunks(&mut self) {
    self.tree.clear_data();
  }

  ///
  /// Set the point to consider as the observation point
  /// when computing the LODs
  ///
  /// # Parameters
  ///
  /// * `point` - The location to observe from
  ///
  pub fn set_observation_point(&mut self, point: [f32; 3]) {
    self.observation_point = point.into();
  }

  /// Iter over all leafs in the tree
  #[allow(dead_code)]
  fn iter(&self) -> OctTreeIter<'_, Chunked<C>> {
    OctTreeIter::new(&self.tree)
  }

  /// Iter over all leafs in the tree mutably
  #[allow(dead_code)]
  fn iter_mut(&mut self) -> OctTreeIterMut<'_, Chunked<C>> {
    OctTreeIterMut::new(&mut self.tree)
  }

  /// Iter over all leafs in the tree that are visible
  #[allow(dead_code)]
  fn visible_iter(&self) -> OctTreeVisibleIter<'_, Chunked<C>> {
    OctTreeVisibleIter::new(&self.tree, self.observation_point, self.lod_distance)
  }

  /// Iter over all leafs in the tree mutably that are visible
  #[allow(dead_code)]
  fn visible_iter_mut(&mut self) -> OctTreeVisibleIterMut<'_, Chunked<C>> {
    OctTreeVisibleIterMut::new(&mut self.tree, self.observation_point, self.lod_distance)
  }
}

///
/// OctTree is the internal structure used to manage the
/// chunks.
///
/// Each leaf and node holds data which will be the chunk.
///
///
struct OctTree<Data> {
  value: OctTreeLeaf<Data>,
  /// Octtree children with order
  /// bottom_front_left
  /// bottom_front_right
  /// bottom_back_right
  /// bottom_back_left
  /// top_front_left
  /// top_front_right
  /// top_back_right
  /// top_back_left
  children: Vec<OctTree<Data>>,
}

impl<Data> OctTree<Data> {
  /// Get the bounds (min, max) that this leaf encloses
  fn get_bounds(&self) -> &Boundary {
    self.value.get_bounds()
  }

  /// Set the arbitary data for this leaf
  #[allow(dead_code)]
  fn set_data(&mut self, data: Data) {
    self.value.set_data(data)
  }

  /// Get the arbitary data for this leaf
  #[allow(dead_code)]
  fn get_data(&self) -> Option<&Data> {
    self.value.get_data()
  }

  /// Get the mutable arbitary data for this leaf
  #[allow(dead_code)]
  fn get_data_mut(&mut self) -> Option<&mut Data> {
    self.value.get_data_mut()
  }

  /// Get the center of this leaf
  fn get_center(&self) -> Vec3 {
    self.value.get_center()
  }

  /// Get the size of this leaf
  fn get_size(&self) -> Vec3 {
    self.value.get_size()
  }

  /// Get the origin (min) of the leaf
  #[allow(dead_code)]
  fn get_origin(&self) -> Vec3 {
    self.value.get_origin()
  }

  /// Get the lod level of this leaf. 0 is the highest detail
  fn get_level(&self) -> usize {
    self.value.get_level()
  }

  /// Used to make the new children recursively
  fn make_children(bounds: Boundary, levels: usize) -> Vec<OctTree<Data>> {
    if levels > 0 {
      let new_levels = levels - 1;
      let (min, max) = (bounds.min, bounds.max);
      let minx = min[0];
      let maxx = max[0];
      let half_x = (min[0] + max[0]) / 2.;
      let miny = min[1];
      let maxy = max[1];
      let half_y = (min[1] + max[1]) / 2.;
      let minz = min[2];
      let maxz = max[2];
      let half_z = (min[2] + max[2]) / 2.;
      vec![
        OctTree::new(
          Boundary::new(&[minx, miny, half_z].into(), &[half_x, half_y, maxz].into()),
          new_levels,
        ),
        OctTree::new(
          Boundary::new(&[half_x, miny, half_z].into(), &[maxx, half_y, maxz].into()),
          new_levels,
        ),
        OctTree::new(
          Boundary::new(&[half_x, miny, minz].into(), &[maxx, half_y, half_z].into()),
          new_levels,
        ),
        OctTree::new(
          Boundary::new(&[minx, miny, minz].into(), &[half_x, half_y, half_z].into()),
          new_levels,
        ),
        OctTree::new(
          Boundary::new(&[minx, half_y, half_z].into(), &[half_x, maxy, maxz].into()),
          new_levels,
        ),
        OctTree::new(
          Boundary::new(&[half_x, half_y, half_z].into(), &[maxx, maxy, maxz].into()),
          new_levels,
        ),
        OctTree::new(
          Boundary::new(&[half_x, half_y, minz].into(), &[maxx, maxy, half_z].into()),
          new_levels,
        ),
        OctTree::new(
          Boundary::new(&[minx, half_y, minz].into(), &[half_x, maxy, half_z].into()),
          new_levels,
        ),
      ]
    } else {
      vec![]
    }
  }

  /// Create a new level of the tree. If levels > 0 then also create its children
  /// bounds is the space this tree encloses
  fn new(bounds: Boundary, levels: usize) -> OctTree<Data> {
    let new_children = OctTree::make_children(bounds.clone(), levels);
    let value = OctTreeLeaf::new(bounds, levels);
    OctTree {
      value,
      children: new_children,
    }
  }

  /// This will change the tree such that it includes
  /// the specified number of levels
  fn resize_tree(&mut self, levels: usize) {
    if self.get_level() == levels {
      return; // No change
    } else if self.children.len() == 0 && levels > 0 {
      // growing the tree from here
      let bounds = Boundary::new(&self.get_bounds().min, &self.get_bounds().max);
      let new_children = OctTree::make_children(bounds, levels);
      self.children = new_children;
    } else if levels == 0 {
      // trim the tree here
      self.children = vec![];
    } else {
      for child in self.children.iter_mut() {
        child.resize_tree(levels - 1);
      }
    }
    self.value.level = levels;
  }

  /// This will clear all data on all leafs
  fn clear_data(&mut self) {
    for child in self.children.iter_mut() {
      child.clear_data();
    }
    self.value.clear_data();
  }
}

///
/// This is the leaf data for an OctTree
///
struct OctTreeLeaf<Data> {
  /// Bounds represents the desired size of the domain in (min, max)
  bounds: Boundary,

  /// Arbitary data for this LOD level
  data: Option<Data>,

  /// LOD Level 0 is highest detail
  level: usize,
}

impl<Data> OctTreeLeaf<Data> {
  /// Get the bounds this leaf encloses
  fn get_bounds(&self) -> &Boundary {
    &self.bounds
  }

  /// Set the arbitary data of this leaf
  fn set_data(&mut self, data: Data) {
    self.data = Some(data);
  }

  /// Set the arbitary data of this leaf
  fn clear_data(&mut self) {
    self.data = None;
  }

  /// Get the arbitary data of this leaf
  fn get_data(&self) -> Option<&Data> {
    self.data.as_ref()
  }

  /// Get the mutable arbitary data of this leaf
  fn get_data_mut(&mut self) -> Option<&mut Data> {
    self.data.as_mut()
  }

  /// Get the center of the leaf
  fn get_center(&self) -> Vec3 {
    (self.bounds.min + self.bounds.max) / 2.
  }

  /// Get the size of the leaf
  fn get_size(&self) -> Vec3 {
    self.bounds.max - self.bounds.min
  }

  /// Gets the origin (min) of the leaf
  fn get_origin(&self) -> Vec3 {
    self.bounds.min
  }

  /// Get the lod level of the leaf. 0 represents the highest detail
  fn get_level(&self) -> usize {
    self.level
  }

  /// Create a new leaf without any arbitary data
  fn new(bounds: Boundary, levels: usize) -> OctTreeLeaf<Data> {
    let data: Option<Data> = None;

    let leaf = OctTreeLeaf {
      bounds: bounds,
      data,
      level: levels,
    };
    leaf
  }
}

/// This is the iter of a tree.
/// It visits every leaf
struct OctTreeIter<'a, Data> {
  stack: Vec<&'a OctTree<Data>>,
}

impl<'a, Data> OctTreeIter<'a, Data> {
  fn new(tree: &'a OctTree<Data>) -> OctTreeIter<'a, Data> {
    OctTreeIter { stack: vec![tree] }
  }
}

impl<'a, Data> Iterator for OctTreeIter<'a, Data> {
  type Item = &'a OctTreeLeaf<Data>;

  fn next(&mut self) -> Option<Self::Item> {
    let node = self.stack.pop()?;
    for child in node.children.iter() {
      self.stack.push(child);
    }
    return Some(&node.value);
  }
}

/// This is the mutable iter of a tree.
/// It visits every leaf
struct OctTreeIterMut<'a, Data> {
  stack: Vec<&'a mut OctTree<Data>>,
}

impl<'a, Data> OctTreeIterMut<'a, Data> {
  fn new(tree: &'a mut OctTree<Data>) -> OctTreeIterMut<'a, Data> {
    OctTreeIterMut { stack: vec![tree] }
  }
}

impl<'a, Data> Iterator for OctTreeIterMut<'a, Data> {
  type Item = &'a mut OctTreeLeaf<Data>;

  fn next(&mut self) -> Option<Self::Item> {
    let node = self.stack.pop()?;
    for child in node.children.iter_mut() {
      self.stack.push(child);
    }
    return Some(&mut node.value);
  }
}

/// This is the visible iter of a tree.
/// It visits only leafs that are visible
struct OctTreeVisibleIter<'a, Data> {
  stack: Vec<&'a OctTree<Data>>,
  camera_position: Vec3,
  lod_distance: f32,
}

impl<'a, Data> OctTreeVisibleIter<'a, Data> {
  fn new(
    tree: &'a OctTree<Data>,
    camera_position: Vec3,
    lod_distance: f32,
  ) -> OctTreeVisibleIter<'a, Data> {
    OctTreeVisibleIter {
      stack: vec![tree],
      camera_position,
      lod_distance,
    }
  }
}

impl<'a, Data> Iterator for OctTreeVisibleIter<'a, Data> {
  type Item = &'a OctTreeLeaf<Data>;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      let node = self.stack.pop()?;
      if node.children.len() > 0 {
        let center = node.get_center();
        let d = center.distance(self.camera_position);
        let lod: usize = ((d / self.lod_distance).ln() / 2_f32.ln()).floor() as usize;
        if node.get_level() <= lod {
          return Some(&node.value);
        } else {
          for child in node.children.iter() {
            self.stack.push(child);
          }
        }
      } else {
        return Some(&node.value);
      }
    }
  }
}

/// This is the mutable visible iter of a tree.
/// It visits only leafs that are visible
struct OctTreeVisibleIterMut<'a, Data> {
  stack: Vec<&'a mut OctTree<Data>>,
  camera_position: Vec3,
  lod_distance: f32,
}

impl<'a, Data> OctTreeVisibleIterMut<'a, Data> {
  fn new(
    tree: &'a mut OctTree<Data>,
    camera_position: Vec3,
    lod_distance: f32,
  ) -> OctTreeVisibleIterMut<'a, Data> {
    OctTreeVisibleIterMut {
      stack: vec![tree],
      camera_position,
      lod_distance,
    }
  }
}

impl<'a, Data> Iterator for OctTreeVisibleIterMut<'a, Data> {
  type Item = &'a mut OctTreeLeaf<Data>;

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      let node = self.stack.pop()?;
      if node.children.len() > 0 {
        let center = node.get_center();
        let d = center.distance(self.camera_position);
        let lod: usize = ((d / self.lod_distance).ln() / 2_f32.ln()).floor() as usize;
        if node.get_level() <= lod {
          return Some(&mut node.value);
        } else {
          for child in node.children.iter_mut() {
            self.stack.push(child);
          }
        }
      } else {
        return Some(&mut node.value);
      }
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::chunk::Chunk;

  #[test]
  fn test_lod_chunker() {
    let dimensions = [48, 48, 48];
    let data = vec![1.; dimensions[0] * dimensions[1] * dimensions[2]];

    let mut chunker: LodChunker<Chunk> =
      LodChunker::from_array(&data, dimensions[0], dimensions[1], dimensions[2]);
    chunker.set_chunk_size([16, 16, 16]);
    chunker.set_lod_distance(32.);
    chunker.set_observation_point([10., 1., -1.]);
    chunker.set_lod_levels(2);

    chunker.generate_chunks();
    let results = chunker.get_chunks();

    assert_eq!(results.len(), 64);
  }
}
