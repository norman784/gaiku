use crate::boundary::Boundary;
use std::{
  collections::HashSet,
  convert::TryInto,
  sync::{Arc, Mutex},
};

/// Base common denominator across all the mesh implementations used.
pub trait Meshify {
  fn new() -> Self;
  fn with(
    indices: Vec<u32>,
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
  ) -> Self;
  fn get_indices(&self) -> &Vec<u32>;
  fn get_normals(&self) -> &Vec<[f32; 3]>;
  fn get_positions(&self) -> &Vec<[f32; 3]>;
  fn get_uvs(&self) -> &Vec<[f32; 2]>;
  fn set_indices(&mut self, indices: Vec<u32>);
  fn set_normals(&mut self, normals: Vec<[f32; 3]>);
  fn set_positions(&mut self, positions: Vec<[f32; 3]>);
  fn set_uvs(&mut self, uvs: Vec<[f32; 2]>);
}

/// Provides a `Meshify` implementation width indices, normals, positions and uvs.
#[derive(Debug)]
pub struct Mesh {
  indices: Vec<u32>,
  normals: Vec<[f32; 3]>,
  positions: Vec<[f32; 3]>,
  uvs: Vec<[f32; 2]>,
}

impl Default for Mesh {
  fn default() -> Self {
    Self {
      indices: vec![],
      normals: vec![],
      positions: vec![],
      uvs: vec![],
    }
  }
}

impl Meshify for Mesh {
  fn new() -> Self {
    Default::default()
  }

  fn with(
    indices: Vec<u32>,
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
  ) -> Self {
    Mesh {
      indices,
      positions,
      normals,
      uvs,
    }
  }

  fn get_indices(&self) -> &Vec<u32> {
    &self.indices
  }

  fn get_normals(&self) -> &Vec<[f32; 3]> {
    &self.normals
  }

  fn get_positions(&self) -> &Vec<[f32; 3]> {
    &self.positions
  }

  fn get_uvs(&self) -> &Vec<[f32; 2]> {
    &self.uvs
  }

  fn set_indices(&mut self, indices: Vec<u32>) {
    self.indices = indices;
  }

  fn set_normals(&mut self, normals: Vec<[f32; 3]>) {
    self.normals = normals;
  }

  fn set_positions(&mut self, positions: Vec<[f32; 3]>) {
    self.positions = positions;
  }

  fn set_uvs(&mut self, uvs: Vec<[f32; 2]>) {
    self.uvs = uvs;
  }
  /*
    /// This will generate a texture from the
    /// mesh vertex colors and update the UV map
    /// Assumes a face has only one color.
    /// The result is an array of u32 which represents
    /// the RGBA color
    pub fn generate_texture(&mut self, width: usize, height: usize) -> Vec<u32> {
      let mut colors: HashMap<(u8, u8, u8, u8), Vec<[u16; 3]>> = HashMap::new();
      self.uv = vec![Vector2 { x: 0., y: 0. }; self.vertices.len()];

      // Check the color of every face
      // Insert this color (as the key) into a dictionary
      // for the value store the face indices
      for face in self.indices.chunks(3) {
        let (f1, f2, f3) = (face[0], face[1], face[2]);
        let color = self.colors[f1 as usize];
        let verts_for_color = colors
          .entry((color.x, color.y, color.z, color.w))
          .or_insert(vec![]);
        verts_for_color.push([f1, f2, f3]);
      }

      let current_num = colors.len();
      // TODO: Currently we just use the sqrt so that there
      // are equal number of colors in the rows and columns
      // really we should use the width:height ratio for
      // optimial placement.
      let colors_in_x = ((current_num as f32).sqrt()) as usize;
      let colors_in_y = current_num / colors_in_x;
      let scale_x = (colors_in_x + 1) as f32;
      let scale_y = (colors_in_y + 1) as f32;

      let mut i = 0;
      let mut result: Vec<u32> = vec![0; width * height];
      // For each color and list of faces
      // Assign a uv coordinate
      //  This coordinate maps to a square in the texture
      // Then blit that area of the result (which is an array of colors representing the image)
      //   with the color.
      for (color, faces) in colors {
        let x_pos = (i % colors_in_x) as f32;
        let y_pos = (i / colors_in_x) as f32;
        // Update all faces with this UV
        for face in faces {
          let (i1, i2, i3) = (face[0], face[1], face[2]);
          self.uv[i1 as usize] = Vector2 {
            x: (x_pos + 0.01) / scale_x,
            y: (y_pos + 0.01) / scale_y,
          };
          self.uv[i2 as usize] = Vector2 {
            x: (x_pos + 1.0 - 0.01) / scale_x,
            y: (y_pos + 0.01) / scale_y,
          };
          self.uv[i3 as usize] = Vector2 {
            x: (x_pos + 1.0 - 0.01) / scale_x,
            y: (y_pos + 1.0 - 0.01) / scale_y,
          };
        }
        // Blit the texture with the color
        let y_start_tex = (y_pos / scale_y * (height as f32)) as usize;
        let y_end_tex = ((y_pos + 1.) / scale_y * (height as f32)) as usize;
        let x_start_tex = (x_pos / scale_x * (width as f32)) as usize;
        let x_end_tex = ((x_pos + 1.) / scale_x * (width as f32)) as usize;
        for y in y_start_tex..y_end_tex {
          for x in x_start_tex..x_end_tex {
            let i = y * width + x;
            let (r, g, b, a) = color;
            result[i] = u32::from_le_bytes([r, g, b, a]);
          }
        }
        i += 1;
      }
      // Return the texture
      return result;
    }
  */
}

#[derive(Debug)]
enum MeshBuilderOctreeNode {
  Leaf(Vec<([f32; 3], Boundary, usize)>),
  Subtree(Box<[MeshBuilderOctree; 8]>),
}

enum InsertResult {
  AlreadyExists(u32),
  FailedInsert,
  Inserted(u32),
  OutOfBounds,
}

struct MeshBuilderOctree {
  boundary: Boundary,
  bucket: usize,
  node: MeshBuilderOctreeNode,
  split_at: usize,
  current_index: Arc<Mutex<usize>>,
}

impl MeshBuilderOctree {
  fn new(boundary: Boundary, bucket: usize, split_at: usize) -> Self {
    Self::new_with_index(boundary, bucket, split_at, Arc::new(Mutex::new(0)))
  }

  fn new_with_index(
    boundary: Boundary,
    bucket: usize,
    split_at: usize,
    index: Arc<Mutex<usize>>,
  ) -> Self {
    Self {
      boundary,
      bucket,
      node: MeshBuilderOctreeNode::Leaf(vec![]),
      split_at,
      current_index: index,
    }
  }

  fn insert(&mut self, leaf: &[f32; 3]) -> InsertResult {
    if self.boundary.contains(&leaf.clone().into()) {
      match &mut self.node {
        MeshBuilderOctreeNode::Leaf(leafs) => {
          for (_, position, index) in leafs.iter() {
            if position.contains(&leaf.clone().into()) {
              return InsertResult::AlreadyExists((*index).try_into().unwrap());
            }
          }

          let boundary = Boundary::new(leaf.clone(), [1e-5, 1e-5, 1e-5]);
          let mut current_index = (*self.current_index).lock().unwrap();
          let new_index = *current_index;
          leafs.push((leaf.clone(), boundary, new_index));
          *current_index += 1;

          if leafs.len() > self.split_at && self.bucket > 0 {
            let leafs = leafs.clone();
            let mut nodes = subdivide(
              &self.boundary,
              self.bucket,
              self.split_at,
              self.current_index.clone(),
            );
            for (leaf, _, _) in leafs.iter() {
              for node in nodes.iter_mut() {
                if let InsertResult::Inserted(_) = node.insert(leaf) {
                  break;
                }
              }
            }

            self.node = MeshBuilderOctreeNode::Subtree(nodes);
          }

          InsertResult::Inserted(new_index.try_into().unwrap())
        }
        MeshBuilderOctreeNode::Subtree(nodes) => {
          for node in nodes.iter_mut() {
            match node.insert(leaf) {
              InsertResult::Inserted(index) => return InsertResult::Inserted(index),
              InsertResult::AlreadyExists(index) => return InsertResult::AlreadyExists(index),
              InsertResult::FailedInsert => return InsertResult::FailedInsert,
              _ => {}
            }
          }

          InsertResult::FailedInsert
        }
      }
    } else {
      InsertResult::OutOfBounds
    }
  }

  fn get_all_ww_index(&self) -> Vec<([f32; 3], usize)> {
    // Unlike get_all this returns unsorted but with the index included
    match &self.node {
      MeshBuilderOctreeNode::Leaf(leafs) => {
        // Get data + index to sort by
        leafs
          .iter()
          .map(|(d, _, index)| (d.clone(), *index))
          .collect::<Vec<_>>()
      }
      MeshBuilderOctreeNode::Subtree(nodes) => nodes
        .iter()
        .map(|n| n.get_all_ww_index())
        .flatten()
        .collect::<Vec<_>>(),
    }
  }

  fn get_all(&self) -> Vec<[f32; 3]> {
    // This gets all data sorted by index

    // Get all data with the index
    let mut raw_table = self.get_all_ww_index();

    // sort it by that index
    raw_table.sort_by(|(_, a_index), (_, b_index)| a_index.partial_cmp(&b_index).unwrap());

    // return the sorted data only not the index
    raw_table.iter().map(|(d, _)| d.clone()).collect::<Vec<_>>()
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

/// Helper component that makes easy to build a triangle list mesh.
#[derive(Debug)]
pub struct MeshBuilder {
  unique_nodes: HashSet<NodeIndices>,
  vertex_cache: MeshBuilderOctree,
  normal_cache: MeshBuilderOctree,
  uv_cache: MeshBuilderOctree,
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct NodeIndices {
  vertex: usize,
  normal: Option<usize>,
  uv: Option<usize>,
  atlas: u16,
}

impl MeshBuilder {
  /// Crates a new mesh centered at a position and size.
  pub fn create(center: [f32; 3], size: [f32; 3]) -> Self {
    Self {
      unique_nodes: HashSet::new(),
      vertex_cache: MeshBuilderOctree::new(Boundary::new(center, size), 3, 25),
      normal_cache: MeshBuilderOctree::new(Boundary::new([0., 0., 0.], [2., 2., 2.]), 3, 25),
      uv_cache: MeshBuilderOctree::new(Boundary::new([0.5, 0.5, 0.5], [1., 1., 1.]), 3, 25),
    }
  }

  /// Inserts the vertice (position, normal, uv and atlas_index) if doesn't exists
  /// and create a new indice for the current data, otherwise retrieves the index of
  /// the input data and inserts the existing index.
  pub fn add(
    &mut self,
    position: [f32; 3],
    normal: Option<[f32; 3]>,
    uv: Option<[f32; 2]>,
    atlas_index: u16,
  ) {
    let vertex_index = match self.vertex_cache.insert(&position) {
      InsertResult::Inserted(index) => index.try_into().unwrap(),
      InsertResult::AlreadyExists(index) => index.try_into().unwrap(),
      InsertResult::FailedInsert => panic!("Failed to insert position {:?}", position),
      InsertResult::OutOfBounds => panic!("Out of bounds position {:?}", position),
    };
    let normal_index = if let Some(normal) = normal {
      match self.normal_cache.insert(&normal) {
        InsertResult::Inserted(index) => Some(index.try_into().unwrap()),
        InsertResult::AlreadyExists(index) => Some(index.try_into().unwrap()),
        InsertResult::FailedInsert => panic!("Failed to insert normal {:?}", normal),
        InsertResult::OutOfBounds => panic!("Out of bounds normal {:?}", normal),
      }
    } else {
      None
    };
    let uv_index = if let Some(uv) = uv {
      match self.normal_cache.insert(&[uv[0], uv[1], 0.]) {
        // Ignore w coordinate for now
        InsertResult::Inserted(index) => Some(index.try_into().unwrap()),
        InsertResult::AlreadyExists(index) => Some(index.try_into().unwrap()),
        InsertResult::FailedInsert => panic!("Failed to insert uv {:?}", uv),
        InsertResult::OutOfBounds => panic!("Out of bounds uv {:?}", uv),
      }
    } else {
      None
    };

    self.unique_nodes.insert(NodeIndices {
      vertex: vertex_index,
      normal: normal_index,
      uv: uv_index,
      atlas: atlas_index,
    });
  }

  /// Inserts the triangle and generate the index if needed, otherwise use an existing index.
  /// The triangle data is expected to be counter-clockwise.
  pub fn add_triangle(
    &mut self,
    triangle: [[f32; 3]; 3],
    normal: Option<[f32; 3]>,
    uv: Option<[[f32; 2]; 3]>,
    atlas_index: u16,
  ) {
    for (i, vertex) in triangle.iter().enumerate() {
      self.add(
        *vertex,
        normal,
        if let Some(uv) = uv { Some(uv[i]) } else { None },
        atlas_index,
      );
    }
  }

  /// Inserts the face (generates 2  triangles) and generate the index if needed,
  /// otherwise use an existing index. The face data is expected to be counter-clockwise.
  pub fn add_face(
    &mut self,

    face: [[f32; 3]; 4],
    normal: Option<[f32; 3]>,
    uv: Option<[[f32; 2]; 4]>,
    atlas_index: u16,
  ) {
    [[0, 1, 3], [1, 2, 3]].iter().for_each(|triangle| {
      triangle.iter().for_each(|i| {
        self.add(
          face[*i],
          normal,
          if let Some(uv) = uv {
            Some(uv[*i])
          } else {
            None
          },
          atlas_index,
        );
      });
    });
  }

  pub fn build<M>(&self) -> Option<M>
  where
    M: Meshify,
  {
    if !self.unique_nodes.is_empty() {
      let vertex_table = self.vertex_cache.get_all();
      let normal_table = self.normal_cache.get_all();
      let uv_table = self.uv_cache.get_all();

      // Iter of a hashset is in arbitary order so we collect to ensure order is fixed for rest of build
      let unique_nodes: Vec<NodeIndices> = self.unique_nodes.iter().cloned().collect();

      let indices = unique_nodes
        .iter()
        .enumerate()
        .map(|(i, _)| i.try_into().unwrap())
        .collect();
      let positions = unique_nodes
        .iter()
        .map(|d| vertex_table[d.vertex].clone())
        .collect();
      let normals = unique_nodes
        .iter()
        .filter(|d| d.normal.is_some()) // Might be better to use a dud value like [0.,0.,0.] instead
        .map(|d| normal_table[d.normal.unwrap()].clone())
        .collect();
      let uvs = unique_nodes
        .iter()
        .filter(|d| d.uv.is_some()) // Might be better to use a dud value like [0.,0.] instead
        .map(|d| {
          let uvw = uv_table[d.uv.unwrap()].clone();
          [uvw[0], uvw[2]]
        })
        .collect();

      Some(M::with(indices, positions, normals, uvs))
    } else {
      None
    }
  }
}

impl Default for MeshBuilder {
  fn default() -> Self {
    MeshBuilder::create([0.0, 0.0, 0.0], [40.0, 40.0, 40.0])
  }
}

#[allow(clippy::many_single_char_names)]
fn subdivide(
  boundary: &Boundary,
  bucket: usize,
  split_at: usize,
  current_index: Arc<Mutex<usize>>,
) -> Box<[MeshBuilderOctree; 8]> {
  let w = boundary.size.x / 2.0;
  let h = boundary.size.y / 2.0;
  let d = boundary.size.z / 2.0;
  let size: [f32; 3] = [w, h, d];
  let hw = size[0] / 2.0;
  let hh = size[1] / 2.0;
  let hd = size[2] / 2.0;

  let x = boundary.center.x;
  let y = boundary.center.y;
  let z = boundary.center.z;

  let new_bucket = bucket - 1;

  Box::new([
    MeshBuilderOctree::new_with_index(
      Boundary::new([x - hw, y + hh, z + hd], size),
      new_bucket,
      split_at,
      current_index.clone(),
    ),
    MeshBuilderOctree::new_with_index(
      Boundary::new([x + hw, y + hh, z + hd], size),
      new_bucket,
      split_at,
      current_index.clone(),
    ),
    MeshBuilderOctree::new_with_index(
      Boundary::new([x - hw, y + hh, z - hd], size),
      new_bucket,
      split_at,
      current_index.clone(),
    ),
    MeshBuilderOctree::new_with_index(
      Boundary::new([x + hw, y + hh, z - hd], size),
      new_bucket,
      split_at,
      current_index.clone(),
    ),
    MeshBuilderOctree::new_with_index(
      Boundary::new([x - hw, y - hh, z + hd], size),
      new_bucket,
      split_at,
      current_index.clone(),
    ),
    MeshBuilderOctree::new_with_index(
      Boundary::new([x + hw, y - hh, z + hd], size),
      new_bucket,
      split_at,
      current_index.clone(),
    ),
    MeshBuilderOctree::new_with_index(
      Boundary::new([x - hw, y - hh, z - hd], size),
      new_bucket,
      split_at,
      current_index.clone(),
    ),
    MeshBuilderOctree::new_with_index(
      Boundary::new([x + hw, y - hh, z - hd], size),
      new_bucket,
      split_at,
      current_index.clone(),
    ),
  ])
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_octree_subtrees_size() {
    let mut tree = MeshBuilderOctree::new(Boundary::new([0.0, 0.0, 0.0], [4.0, 4.0, 4.0]), 3, 5);

    for x in 0..4 {
      for y in 0..4 {
        for z in 0..4 {
          tree.insert([x as f32, y as f32, z as f32]);
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
    let mut tree =
      MeshBuilderOctree::new(Boundary::new([0.0, 0.0, 0.0], [16.0, 16.0, 16.0]), 3, 25);

    match tree.insert(&[0.0, 0.0, 0.0]) {
      InsertResult::Inserted => assert!(true),
      _ => assert!(false),
    }

    assert_eq!(tree.get_all().len(), 1);
  }

  #[test]
  fn test_octree_insert_edge_case() {
    let mut tree =
      MeshBuilderOctree::new(Boundary::new([8.0, 8.0, 8.0], [16.0, 16.0, 16.0]), 3, 25);

    match tree.insert(&[3.5, 16.0, 12.5]) {
      InsertResult::Inserted => assert!(true),
      _ => {
        println!("{:#?}", &tree);
        assert!(false)
      }
    }

    assert_eq!(tree.get_all().len(), 1);
  }
}
