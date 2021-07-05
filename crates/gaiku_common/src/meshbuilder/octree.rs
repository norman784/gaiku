use super::MeshBuilder;
use crate::{boundary::Boundary, mesh::Meshify};
use glam::{Vec2, Vec3};

const EPSILON: f32 = 1e-4;

/// Helper component that makes easy to build a triangle list mesh.
#[derive(Debug)]
pub struct OctMeshBuilder {
  current_index: u32,
  indices: Vec<u32>,
  cache: MeshBuilderOctree,
}

impl MeshBuilder for OctMeshBuilder {
  /// Crates a new mesh centered at a position and size.
  fn create(center: [f32; 3], size: [f32; 3]) -> Self {
    let delta = Vec3::from(size) / (2. - EPSILON * 2.);
    let center: Vec3 = center.into();
    // We set the number of bins such that the miniumum width is constant
    const BIN_MIN_WIDTH: f32 = 0.0125;
    let size_min = delta.min_element() * 2.;
    // width/(2^n) = min_width, n=number of bins
    // n = ln(width/min_width)/ln(2) = n
    let n = std::cmp::max(
      (((size_min / BIN_MIN_WIDTH).ln() / 2_f32.ln()).ceil()) as usize,
      3,
    );

    Self {
      current_index: 0,
      indices: vec![],
      cache: MeshBuilderOctree::new(Boundary::new(&(center - delta), &(center + delta)), n, 25),
    }
  }

  /// Inserts the vertice (position, normal, uv and atlas_index) if doesn't exists
  /// and create a new indice for the current data, otherwise retrieves the index of
  /// the input data and inserts the existing index.
  fn add(
    &mut self,
    position: [f32; 3],
    normal: Option<[f32; 3]>,
    uv: Option<[f32; 2]>,
    atlas_index: u16,
  ) {
    let p: Vec3 = position.into();
    let delta: Vec3 = [EPSILON, EPSILON, EPSILON].into();
    let boundary = Boundary::new(&(p - delta), &(p + delta));

    let new = MeshBuilderData::new(
      position.into(),
      normal.map(|d| d.into()),
      uv.map(|d| d.into()),
      atlas_index,
      self.current_index,
    );

    let found_index = {
      let mut found = None;
      for item in self.cache.find_within(boundary) {
        if item == &new {
          found = Some(item.index);
          break;
        }
      }
      found
    };

    if let Some(found_index) = found_index {
      self.indices.push(found_index);
    } else {
      match self.cache.insert(new) {
        InsertResult::Inserted => {
          self.indices.push(self.current_index);
          self.current_index += 1;
        }
        InsertResult::FailedInsert => panic!("Failed to insert"),
        InsertResult::OutOfBounds => {
          panic!("Out of bounds {:?} in {:?}", position, self.cache.boundary)
        }
      };
    }
  }

  fn build<M>(&self) -> Option<M>
  where
    M: Meshify,
  {
    if !self.indices.is_empty() {
      // Load all data into the rstar tree
      // All at once (this is faster then inceremental instertion)
      let mut unsorted_verts: Vec<_> = self
        .cache
        .iter()
        .map(|data| (data.index, data.clone()))
        .collect();
      unsorted_verts.sort_by_key(|data| data.0);
      let verts: Vec<_> = unsorted_verts.into_iter().map(|data| data.1).collect();

      let indices = self.indices.clone();
      let positions: Vec<_> = verts.iter().map(|d| d.position.into()).collect();
      let normals: Vec<_> = verts
        .iter()
        .filter_map(|d| d.normal.map(|d| d.into()))
        .collect();
      let uvs: Vec<_> = verts
        .iter()
        .filter_map(|d| d.uv.map(|d| d.into()))
        .collect();

      Some(M::with(indices, positions, normals, uvs))
    } else {
      None
    }
  }
}

impl Default for OctMeshBuilder {
  fn default() -> Self {
    MeshBuilder::create([0.0, 0.0, 0.0], [40.0, 40.0, 40.0])
  }
}

#[derive(Clone, Debug)]
struct MeshBuilderData {
  position: Vec3,
  normal: Option<Vec3>,
  uv: Option<Vec2>,
  atlas_index: u16,
  index: u32,
}

impl PartialEq for MeshBuilderData {
  fn eq(&self, other: &Self) -> bool {
    let pos = (self.position - other.position).length() <= EPSILON;
    if !pos {
      return false;
    }

    let normal = match (self.normal, other.normal) {
      (None, None) => true,
      (None, Some(_)) => false,
      (Some(_), None) => false,
      (Some(a), Some(b)) => (a - b).length() <= EPSILON,
    };
    if !normal {
      return false;
    }

    let uv = match (self.uv, other.uv) {
      (None, None) => true,
      (None, Some(_)) => false,
      (Some(_), None) => false,
      (Some(a), Some(b)) => (a - b).length() <= EPSILON,
    };
    if !uv {
      return false;
    }

    self.atlas_index == other.atlas_index
  }
}
impl Eq for MeshBuilderData {}

impl MeshBuilderData {
  fn new(
    position: Vec3,
    normal: Option<Vec3>,
    uv: Option<Vec2>,
    atlas_index: u16,
    index: u32,
  ) -> Self {
    MeshBuilderData {
      position,
      normal,
      uv,
      atlas_index,
      index,
    }
  }
}

#[derive(Debug)]
enum MeshBuilderOctreeNode {
  Leaf(Vec<MeshBuilderData>),
  Subtree(Box<[MeshBuilderOctree; 8]>),
}

#[derive(Debug)]
enum InsertResult {
  FailedInsert,
  Inserted,
  OutOfBounds,
}

struct MeshBuilderOctree {
  boundary: Boundary,
  bucket: usize,
  node: MeshBuilderOctreeNode,
  split_at: usize,
}

impl MeshBuilderOctree {
  fn new(boundary: Boundary, bucket: usize, split_at: usize) -> Self {
    Self {
      boundary,
      bucket,
      node: MeshBuilderOctreeNode::Leaf(vec![]),
      split_at,
    }
  }

  fn insert(&mut self, leaf: MeshBuilderData) -> InsertResult {
    if self.boundary.contains(&leaf.position) {
      match &mut self.node {
        MeshBuilderOctreeNode::Leaf(leafs) => {
          leafs.push(leaf);

          if leafs.len() > self.split_at && self.bucket > 0 {
            let mut nodes = subdivide(&self.boundary, self.bucket, self.split_at);
            for leaf in leafs.drain(..) {
              let insert_to = nodes
                .iter_mut()
                .find(|n| n.boundary.contains(&leaf.position));
              if let Some(insert_to) = insert_to {
                insert_to.insert(leaf);
              }
            }

            self.node = MeshBuilderOctreeNode::Subtree(nodes);
          }

          InsertResult::Inserted
        }
        MeshBuilderOctreeNode::Subtree(nodes) => {
          let insert_to = nodes
            .iter_mut()
            .find(|n| n.boundary.contains(&leaf.position));
          if let Some(insert_to) = insert_to {
            match insert_to.insert(leaf) {
              InsertResult::Inserted => InsertResult::Inserted,
              InsertResult::FailedInsert => InsertResult::FailedInsert,
              _ => InsertResult::FailedInsert,
            }
          } else {
            InsertResult::FailedInsert
          }
        }
      }
    } else {
      InsertResult::OutOfBounds
    }
  }

  fn iter(&self) -> OctreeIter {
    OctreeIter::new(self)
  }

  fn find_within(&self, boundary: Boundary) -> OctreeRangeIter {
    OctreeRangeIter::new(self, boundary)
  }
}

/// This is the iter of a tree.
/// It visits every leaf
struct OctreeIter<'a> {
  stack: Vec<&'a MeshBuilderOctree>,
  stack_data: Vec<&'a MeshBuilderData>,
}

impl<'a> OctreeIter<'a> {
  fn new(tree: &'a MeshBuilderOctree) -> OctreeIter<'a> {
    OctreeIter {
      stack: vec![tree],
      stack_data: vec![],
    }
  }
}

impl<'a> Iterator for OctreeIter<'a> {
  type Item = &'a MeshBuilderData;

  fn next(&mut self) -> Option<Self::Item> {
    while self.stack_data.is_empty() && !self.stack.is_empty() {
      let tree = self.stack.pop();
      if let Some(node) = tree.map(|t| &t.node) {
        match node {
          MeshBuilderOctreeNode::Subtree(trees) => {
            for tree in trees.iter() {
              self.stack.push(tree);
            }
          }
          MeshBuilderOctreeNode::Leaf(leaves) => {
            for leaf in leaves.iter() {
              self.stack_data.push(leaf);
            }
          }
        }
      }
    }
    self.stack_data.pop()
  }
}

/// This iter visits every leaf in side a boundary
struct OctreeRangeIter<'a> {
  stack: Vec<&'a MeshBuilderOctree>,
  stack_data: Vec<&'a MeshBuilderData>,
  boundary: Boundary,
}

impl<'a> OctreeRangeIter<'a> {
  fn new(tree: &'a MeshBuilderOctree, boundary: Boundary) -> OctreeRangeIter<'a> {
    let data = if tree.boundary.overlaps(&boundary) {
      vec![tree]
    } else {
      vec![]
    };
    OctreeRangeIter {
      stack: data,
      stack_data: vec![],
      boundary,
    }
  }
}

impl<'a> Iterator for OctreeRangeIter<'a> {
  type Item = &'a MeshBuilderData;

  fn next(&mut self) -> Option<Self::Item> {
    while self.stack_data.is_empty() && !self.stack.is_empty() {
      let tree = self.stack.pop();
      if let Some(node) = tree.map(|t| &t.node) {
        match node {
          MeshBuilderOctreeNode::Subtree(trees) => {
            for tree in trees.iter() {
              if tree.boundary.overlaps(&self.boundary) {
                self.stack.push(tree);
              }
            }
          }
          MeshBuilderOctreeNode::Leaf(leaves) => {
            for leaf in leaves.iter() {
              if self.boundary.contains(&leaf.position) {
                self.stack_data.push(leaf);
              }
            }
          }
        }
      }
    }
    self.stack_data.pop()
  }
}

impl std::fmt::Debug for MeshBuilderOctree {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("MeshBuilderOctree")
      .field("boundary", &self.boundary)
      .field("bucket", &self.bucket)
      .field("nodes", &self.node)
      .finish()
  }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Position(i32, i32, i32);

impl From<[f32; 3]> for Position {
  fn from([x, y, z]: [f32; 3]) -> Self {
    Position(
      (x * 1_000_000.0) as i32,
      (y * 1_000_000.0) as i32,
      (z * 1_000_000.0) as i32,
    )
  }
}

#[allow(clippy::many_single_char_names)]
fn subdivide(boundary: &Boundary, bucket: usize, split_at: usize) -> Box<[MeshBuilderOctree; 8]> {
  let p1 = boundary.min;
  let p2 = (boundary.min + boundary.max) / 2.;
  let min_x = p1[0];
  let min_y = p1[1];
  let min_z = p1[2];
  let max_x = p2[0];
  let max_y = p2[1];
  let max_z = p2[2];

  let mins: [Vec3; 8] = [
    [min_x, min_y, min_z].into(),
    [max_x, min_y, min_z].into(),
    [max_x, max_y, min_z].into(),
    [min_x, max_y, min_z].into(),
    [min_x, min_y, max_z].into(),
    [max_x, min_y, max_z].into(),
    [max_x, max_y, max_z].into(),
    [min_x, max_y, max_z].into(),
  ];

  let p1 = p2;
  let p2 = boundary.max;
  let min_x = p1[0];
  let min_y = p1[1];
  let min_z = p1[2];
  let max_x = p2[0];
  let max_y = p2[1];
  let max_z = p2[2];

  let maxs: [Vec3; 8] = [
    [min_x, min_y, min_z].into(),
    [max_x, min_y, min_z].into(),
    [max_x, max_y, min_z].into(),
    [min_x, max_y, min_z].into(),
    [min_x, min_y, max_z].into(),
    [max_x, min_y, max_z].into(),
    [max_x, max_y, max_z].into(),
    [min_x, max_y, max_z].into(),
  ];

  let new_bucket = bucket - 1;

  Box::new([
    MeshBuilderOctree::new(Boundary::new(&mins[0], &maxs[0]), new_bucket, split_at),
    MeshBuilderOctree::new(Boundary::new(&mins[1], &maxs[1]), new_bucket, split_at),
    MeshBuilderOctree::new(Boundary::new(&mins[2], &maxs[2]), new_bucket, split_at),
    MeshBuilderOctree::new(Boundary::new(&mins[3], &maxs[3]), new_bucket, split_at),
    MeshBuilderOctree::new(Boundary::new(&mins[4], &maxs[4]), new_bucket, split_at),
    MeshBuilderOctree::new(Boundary::new(&mins[5], &maxs[5]), new_bucket, split_at),
    MeshBuilderOctree::new(Boundary::new(&mins[6], &maxs[6]), new_bucket, split_at),
    MeshBuilderOctree::new(Boundary::new(&mins[7], &maxs[7]), new_bucket, split_at),
  ])
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_octree_subtrees_size() {
    let mut tree = MeshBuilderOctree::new(
      Boundary::new(&[0.0, 0.0, 0.0].into(), &[4.0, 4.0, 4.0].into()),
      3,
      5,
    );

    for x in 0..4 {
      for y in 0..4 {
        for z in 0..4 {
          tree.insert(MeshBuilderData::new(
            [x as f32, y as f32, z as f32].into(),
            None,
            None,
            0,
            0,
          ));
        }
      }
    }

    use std::io::prelude::*;
    let contents = format!("{:#?}", tree);
    let path = format!("{}/debug.log", env!["CARGO_MANIFEST_DIR"],);
    let mut file = std::fs::File::create(&path).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
  }

  #[test]
  fn test_octree_insert() {
    let mut tree = MeshBuilderOctree::new(
      Boundary::new(&[0.0, 0.0, 0.0].into(), &[16.0, 16.0, 16.0].into()),
      3,
      25,
    );

    match tree.insert(MeshBuilderData::new(
      [8.0, 8.0, 8.0].into(),
      Some([0.0, 0.0, 0.0].into()),
      Some([0.0, 0.0].into()),
      0,
      0,
    )) {
      InsertResult::Inserted => {}
      _ => panic!(),
    }

    assert_eq!(tree.iter().count(), 1);
  }

  #[test]
  fn test_octree_insert_edge_case() {
    let mut tree = MeshBuilderOctree::new(
      Boundary::new(
        &[8.0, 8.0, 8.0].into(),
        &[16.0 + EPSILON, 16.0 + EPSILON, 16.0 + EPSILON].into(),
      ),
      3,
      25,
    );

    match tree.insert(MeshBuilderData::new(
      [8.0, 8.0, 8.0].into(),
      None,
      None,
      0,
      0,
    )) {
      InsertResult::Inserted => {}
      n => {
        println!("{:?} with {:#?}", n, &tree);
        panic!()
      }
    }

    match tree.insert(MeshBuilderData::new(
      [16.0, 16.0, 16.0].into(),
      None,
      None,
      0,
      0,
    )) {
      InsertResult::Inserted => {}
      n => {
        println!("{:?} with {:#?}", n, &tree);
        panic!()
      }
    }

    assert_eq!(tree.iter().count(), 2);
  }

  #[test]
  fn test_octree_deduplicate() {
    let mut tree = OctMeshBuilder::create([0.0, 0.0, 0.0], [4.0, 4.0, 4.0]);

    for _ in 0..10 {
      tree.add([0., 0., 0.], None, None, 0);
    }

    for _ in 0..10 {
      tree.add([1., 0., 0.], None, None, 0);
    }

    for _ in 0..10 {
      tree.add([1., 1., 0.], None, None, 0);
    }

    for _ in 0..10 {
      tree.add([0., 1., 0.], None, None, 0);
    }

    assert_eq!(tree.cache.iter().count(), 4);
  }
}
