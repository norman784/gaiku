use std::{borrow::Cow, collections::HashMap};

use crate::tree::Boundary;

#[derive(Debug)]
pub enum VertexAttributeValues {
  Float(Vec<f32>),
  Int(Vec<i32>),
  Uint(Vec<u32>),
  Float2(Vec<[f32; 2]>),
  Int2(Vec<[i32; 2]>),
  Uint2(Vec<[u32; 2]>),
  Float3(Vec<[f32; 3]>),
  Int3(Vec<[i32; 3]>),
  Uint3(Vec<[u32; 3]>),
  Float4(Vec<[f32; 4]>),
  Int4(Vec<[i32; 4]>),
  Uint4(Vec<[u32; 4]>),
}

impl VertexAttributeValues {
  pub fn len(&self) -> usize {
    match *self {
      VertexAttributeValues::Float(ref values) => values.len(),
      VertexAttributeValues::Int(ref values) => values.len(),
      VertexAttributeValues::Uint(ref values) => values.len(),
      VertexAttributeValues::Float2(ref values) => values.len(),
      VertexAttributeValues::Int2(ref values) => values.len(),
      VertexAttributeValues::Uint2(ref values) => values.len(),
      VertexAttributeValues::Float3(ref values) => values.len(),
      VertexAttributeValues::Int3(ref values) => values.len(),
      VertexAttributeValues::Uint3(ref values) => values.len(),
      VertexAttributeValues::Float4(ref values) => values.len(),
      VertexAttributeValues::Int4(ref values) => values.len(),
      VertexAttributeValues::Uint4(ref values) => values.len(),
    }
  }

  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }
}

impl From<Vec<f32>> for VertexAttributeValues {
  fn from(vec: Vec<f32>) -> Self {
    VertexAttributeValues::Float(vec)
  }
}

impl From<Vec<i32>> for VertexAttributeValues {
  fn from(vec: Vec<i32>) -> Self {
    VertexAttributeValues::Int(vec)
  }
}

impl From<Vec<u32>> for VertexAttributeValues {
  fn from(vec: Vec<u32>) -> Self {
    VertexAttributeValues::Uint(vec)
  }
}

impl From<Vec<[f32; 2]>> for VertexAttributeValues {
  fn from(vec: Vec<[f32; 2]>) -> Self {
    VertexAttributeValues::Float2(vec)
  }
}

impl From<Vec<[i32; 2]>> for VertexAttributeValues {
  fn from(vec: Vec<[i32; 2]>) -> Self {
    VertexAttributeValues::Int2(vec)
  }
}

impl From<Vec<[u32; 2]>> for VertexAttributeValues {
  fn from(vec: Vec<[u32; 2]>) -> Self {
    VertexAttributeValues::Uint2(vec)
  }
}

impl From<Vec<[f32; 3]>> for VertexAttributeValues {
  fn from(vec: Vec<[f32; 3]>) -> Self {
    VertexAttributeValues::Float3(vec)
  }
}

impl From<Vec<[i32; 3]>> for VertexAttributeValues {
  fn from(vec: Vec<[i32; 3]>) -> Self {
    VertexAttributeValues::Int3(vec)
  }
}

impl From<Vec<[u32; 3]>> for VertexAttributeValues {
  fn from(vec: Vec<[u32; 3]>) -> Self {
    VertexAttributeValues::Uint3(vec)
  }
}

impl From<Vec<[f32; 4]>> for VertexAttributeValues {
  fn from(vec: Vec<[f32; 4]>) -> Self {
    VertexAttributeValues::Float4(vec)
  }
}

impl From<Vec<[i32; 4]>> for VertexAttributeValues {
  fn from(vec: Vec<[i32; 4]>) -> Self {
    VertexAttributeValues::Int4(vec)
  }
}

impl From<Vec<[u32; 4]>> for VertexAttributeValues {
  fn from(vec: Vec<[u32; 4]>) -> Self {
    VertexAttributeValues::Uint4(vec)
  }
}

#[derive(Debug)]
pub enum VertexAttribute {
  Color,
  Normal,
  Position,
  UV,
}

impl VertexAttribute {
  const ATTRIBUTE_COLOR: &'static str = "color";
  const ATTRIBUTE_NORMAL: &'static str = "normal";
  const ATTRIBUTE_POSITION: &'static str = "position";
  const ATTRIBUTE_UV: &'static str = "uv";
}

impl Into<&str> for VertexAttribute {
  fn into(self) -> &'static str {
    use VertexAttribute::*;

    match self {
      Color => VertexAttribute::ATTRIBUTE_COLOR,
      Normal => VertexAttribute::ATTRIBUTE_NORMAL,
      Position => VertexAttribute::ATTRIBUTE_POSITION,
      UV => VertexAttribute::ATTRIBUTE_UV,
    }
  }
}

#[derive(Debug)]
pub enum Indices {
  U16(Vec<u16>),
  U32(Vec<u32>),
}

impl Indices {
  pub fn is_empty(&self) -> bool {
    match self {
      Indices::U16(arr) => arr.is_empty(),
      Indices::U32(arr) => arr.is_empty(),
    }
  }

  pub fn len(&self) -> usize {
    match self {
      Indices::U16(arr) => arr.len(),
      Indices::U32(arr) => arr.len(),
    }
  }
}

#[derive(Debug)]
pub struct Mesh {
  pub indices: Option<Indices>,
  pub attributes: HashMap<Cow<'static, str>, VertexAttributeValues>,
}

impl Default for Mesh {
  fn default() -> Self {
    Self {
      indices: None,
      attributes: HashMap::new(),
    }
  }
}

impl Mesh {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn get_attributes(&self, attribute: VertexAttribute) -> Option<&VertexAttributeValues> {
    self.attributes.get(attribute.into())
  }

  pub fn set_attributes(&mut self, attribute: VertexAttribute, values: VertexAttributeValues) {
    let attribute: &str = attribute.into();
    self.attributes.insert(attribute.into(), values);
  }

  pub fn set_indices(&mut self, indices: Indices) {
    self.indices = Some(indices);
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

#[derive(Clone, Debug)]
struct MeshBuilderData {
  position: [f32; 3],
  normal: Option<[f32; 3]>,
  uv: Option<[f32; 2]>,
  atlas_index: u16,
  index: u32,
}

impl MeshBuilderData {
  fn new(
    position: [f32; 3],
    normal: Option<[f32; 3]>,
    uv: Option<[f32; 2]>,
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

impl From<([f32; 3], Option<[f32; 3]>, Option<[f32; 2]>, u16, u32)> for MeshBuilderData {
  fn from(
    (position, normal, uv, atlas_index, index): (
      [f32; 3],
      Option<[f32; 3]>,
      Option<[f32; 2]>,
      u16,
      u32,
    ),
  ) -> Self {
    MeshBuilderData::new(position, normal, uv, atlas_index, index)
  }
}

#[derive(Debug)]
enum MeshBuilderOctreeNode {
  Leaf(Vec<(MeshBuilderData, Boundary)>),
  Subtree(Box<[MeshBuilderOctree; 8]>),
}

enum InsertResult {
  AlreadyExists(u32),
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

  fn insert(&mut self, leaf: &MeshBuilderData) -> InsertResult {
    if self.boundary.contains(&leaf.position.into()) {
      match &mut self.node {
        MeshBuilderOctreeNode::Leaf(leafs) => {
          for (data, position) in leafs.iter() {
            if position.contains(&leaf.position.into()) {
              return InsertResult::AlreadyExists(data.index);
            }
          }

          let boundary = Boundary::new(leaf.position, [0.00000001, 0.00000001, 0.00000001]);
          leafs.push((leaf.clone(), boundary));

          if leafs.len() > self.split_at && self.bucket > 0 {
            let leafs = leafs.clone();
            let mut nodes = subdivide(&self.boundary, self.bucket, self.split_at);
            for (leaf, _) in leafs.iter() {
              for node in nodes.iter_mut() {
                if let InsertResult::Inserted = node.insert(leaf) {
                  break;
                }
              }
            }

            self.node = MeshBuilderOctreeNode::Subtree(nodes);
          }

          InsertResult::Inserted
        }
        MeshBuilderOctreeNode::Subtree(nodes) => {
          for node in nodes.iter_mut() {
            match node.insert(leaf) {
              InsertResult::Inserted => return InsertResult::Inserted,
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

  fn get_all(&self) -> Vec<MeshBuilderData> {
    match &self.node {
      MeshBuilderOctreeNode::Leaf(leafs) => {
        leafs.iter().map(|(d, _)| d.clone()).collect::<Vec<_>>()
      }
      MeshBuilderOctreeNode::Subtree(nodes) => nodes
        .iter()
        .map(|n| n.get_all())
        .flatten()
        .collect::<Vec<_>>(),
    }
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

#[derive(Debug)]
pub struct MeshBuilder {
  current_index: u32,
  indices: Vec<u32>,
  cache: MeshBuilderOctree,
}

impl MeshBuilder {
  pub fn create(center: [f32; 3], size: f32) -> Self {
    Self {
      current_index: 0,
      indices: vec![],
      cache: MeshBuilderOctree::new(Boundary::new(center, [size, size, size]), 3, 25),
    }
  }

  pub fn add(
    &mut self,
    position: [f32; 3],
    normal: Option<[f32; 3]>,
    uv: Option<[f32; 2]>,
    atlas_index: u16,
  ) {
    let mesh_data = MeshBuilderData::new(position, normal, uv, atlas_index, self.current_index);
    match self.cache.insert(&mesh_data) {
      InsertResult::Inserted => {
        self.indices.push(self.current_index);
        self.current_index += 1;
      }
      InsertResult::AlreadyExists(index) => self.indices.push(index),
      InsertResult::FailedInsert => panic!("Failed to insert {:?}", mesh_data),
      InsertResult::OutOfBounds => {}
    }
  }

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

  /// The face data is expected to be clockwise
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

  pub fn build(&self) -> Option<Mesh> {
    if !self.indices.is_empty() {
      let mut data = self.cache.get_all();
      data.sort_by(|a, b| a.index.partial_cmp(&b.index).unwrap());
      let mut positions = vec![];
      let mut normals = vec![];
      let mut uvs = vec![];

      for row in data.iter() {
        positions.push(row.position);
        if let Some(normal) = row.normal {
          normals.push(normal);
        }

        if let Some(uv) = row.uv {
          uvs.push(uv);
        }
      }

      let mut mesh = Mesh::new();
      mesh.set_indices(Indices::U32(self.indices.clone()));
      mesh.set_attributes(VertexAttribute::Position, positions.into());

      if !normals.is_empty() {
        mesh.set_attributes(VertexAttribute::Normal, normals.into());
      }
      if !uvs.is_empty() {
        mesh.set_attributes(VertexAttribute::UV, uvs.into());
      }

      Some(mesh)
    } else {
      None
    }
  }
}

impl Default for MeshBuilder {
  fn default() -> Self {
    MeshBuilder::create([0.0, 0.0, 0.0], 40.0)
  }
}

#[allow(clippy::many_single_char_names)]
fn subdivide(boundary: &Boundary, bucket: usize, split_at: usize) -> Box<[MeshBuilderOctree; 8]> {
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
    MeshBuilderOctree::new(
      Boundary::new([x - hw, y + hh, z + hd], size),
      new_bucket,
      split_at,
    ),
    MeshBuilderOctree::new(
      Boundary::new([x + hw, y + hh, z + hd], size),
      new_bucket,
      split_at,
    ),
    MeshBuilderOctree::new(
      Boundary::new([x - hw, y + hh, z - hd], size),
      new_bucket,
      split_at,
    ),
    MeshBuilderOctree::new(
      Boundary::new([x + hw, y + hh, z - hd], size),
      new_bucket,
      split_at,
    ),
    MeshBuilderOctree::new(
      Boundary::new([x - hw, y - hh, z + hd], size),
      new_bucket,
      split_at,
    ),
    MeshBuilderOctree::new(
      Boundary::new([x + hw, y - hh, z + hd], size),
      new_bucket,
      split_at,
    ),
    MeshBuilderOctree::new(
      Boundary::new([x - hw, y - hh, z - hd], size),
      new_bucket,
      split_at,
    ),
    MeshBuilderOctree::new(
      Boundary::new([x + hw, y - hh, z - hd], size),
      new_bucket,
      split_at,
    ),
  ])
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_octree_insert() {
    let mut tree =
      MeshBuilderOctree::new(Boundary::new([0.0, 0.0, 0.0], [16.0, 16.0, 16.0]), 3, 25);

    match tree.insert(&MeshBuilderData::new(
      [0.0, 0.0, 0.0],
      Some([0.0, 0.0, 0.0]),
      Some([0.0, 0.0]),
      0,
      0,
    )) {
      InsertResult::Inserted => assert!(true),
      _ => assert!(false),
    }

    assert_eq!(tree.get_all().len(), 1);
  }

  #[test]
  fn test_octree_insert_edge_case() {
    let mut tree =
      MeshBuilderOctree::new(Boundary::new([8.0, 8.0, 8.0], [16.0, 16.0, 16.0]), 3, 25);

    match tree.insert(&MeshBuilderData::new(
      [3.5, 16.352942, 12.5],
      None,
      None,
      0,
      0,
    )) {
      InsertResult::Inserted => assert!(true),
      _ => assert!(false),
    }

    assert_eq!(tree.get_all().len(), 1);
  }
}
