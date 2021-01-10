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
    let attribute: &str = attribute.into();
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
  bbox: ami::BBox,
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
      bbox: ami::BBox::new(
        [position[0] - 0.1, position[1] - 0.1, position[2] - 0.1].into(),
        [position[0] + 0.1, position[1] + 0.1, position[2] + 0.1].into(),
      ),
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

impl ami::Collider for MeshBuilderData {
  fn bbox(&self) -> ami::BBox {
    self.bbox
  }
}

struct MeshBuilderOctree {
  boundary: Boundary,
  bucket: usize,
  leafs: Vec<(MeshBuilderData, Boundary)>,
  nodes: Vec<MeshBuilderOctree>,
}

impl MeshBuilderOctree {
  fn new(boundary: Boundary, bucket: usize) -> Self {
    Self {
      boundary,
      bucket,
      nodes: vec![],
      leafs: vec![],
    }
  }

  fn insert(&mut self, leaf: &MeshBuilderData) -> bool {
    if self.boundary.contains(&leaf.position.into()) {
      if self.nodes.is_empty() {
        if self
          .leafs
          .iter()
          .any(|(_, b)| b.contains(&leaf.position.into()))
        {
          return false;
        } else {
          let boundary = Boundary::new(leaf.position, [0.00000001, 0.00000001, 0.00000001]);
          self.leafs.push((leaf.clone(), boundary));

          if self.leafs.len() > 100 && self.bucket > 0 {
            let leafs = self.leafs.clone();
            self.nodes = subdivide(&self.boundary, self.bucket);
            for (leaf, _) in leafs.iter() {
              for node in self.nodes.iter_mut() {
                if node.insert(leaf) {
                  break;
                }
              }
            }

            self.leafs.clear();
          }

          return true;
        }
      } else {
        for node in self.nodes.iter_mut() {
          if node.insert(leaf) {
            return true;
          }
        }
      }
    }

    false
  }

  fn get(&self, leaf: &MeshBuilderData) -> Option<u32> {
    if self.nodes.is_empty() {
      if let Some((d, _)) = self
        .leafs
        .iter()
        .find(|(_, b)| b.contains(&leaf.position.into()))
      {
        return Some(d.index);
      } else {
        return None;
      }
    } else {
      for node in self.nodes.iter() {
        if let Some(index) = node.get(leaf) {
          return Some(index);
        }
      }
    }

    None
  }

  fn get_all(&self) -> Vec<MeshBuilderData> {
    if self.nodes.is_empty() {
      self
        .leafs
        .iter()
        .map(|(d, _)| d.clone())
        .collect::<Vec<_>>()
    } else {
      let mut res = vec![];

      for node in self.nodes.iter() {
        res.push(node.get_all());
      }

      res.iter().cloned().flatten().collect::<Vec<_>>()
    }
  }
}

impl std::fmt::Debug for MeshBuilderOctree {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("MeshBuilderOctree")
      .field("boundary", &self.boundary)
      .field("bucket", &self.bucket)
      .field("nodes", &self.nodes)
      .field("leafs", &self.leafs.len())
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
  cache2: HashMap<Position, Vec<MeshBuilderData>>,
  cache3: ami::Octree<MeshBuilderData>,
  cache_impl: u8,
}

impl MeshBuilder {
  pub fn create(center: [f32; 3], size: f32) -> Self {
    Self {
      current_index: 0,
      indices: vec![],
      cache: MeshBuilderOctree::new(Boundary::new(center, [size, size, size]), 3),
      cache2: HashMap::new(),
      cache3: ami::Octree::new(),
      cache_impl: 2,
    }
  }

  pub fn add(
    &mut self,
    position: [f32; 3],
    normal: Option<[f32; 3]>,
    uv: Option<[f32; 2]>,
    atlas_index: u16,
  ) {
    if self.cache_impl == 0 {
      let next_index = self.cache2.values().fold(0, |acc, v| acc + v.len()) as u32;
      let data = self.cache2.entry(position.into()).or_insert_with(Vec::new);

      if !data.is_empty() {
        for row in data.iter() {
          if non_precise_eq_vec3_f32(
            row.normal.unwrap_or([0.0, 0.0, 0.0]),
            normal.unwrap_or([0.0, 0.0, 0.0]),
          ) && non_precise_eq_vec2_f32(row.uv.unwrap_or([0.0, 0.0]), uv.unwrap_or([0.0, 0.0]))
            && row.atlas_index == atlas_index
          {
            self.indices.push(row.index);
            return;
          }
        }
      }

      data.push(MeshBuilderData::new(
        position,
        normal,
        uv,
        atlas_index,
        next_index,
      ));

      self.indices.push(next_index);
    } else if self.cache_impl == 1 {
      if let Some(index) =
        self
          .cache
          .get(&MeshBuilderData::new(position, normal, uv, atlas_index, 0))
      {
        self.indices.push(index);
      } else if self.cache.insert(&MeshBuilderData::new(
        position,
        normal,
        uv,
        atlas_index,
        self.current_index,
      )) {
        self.indices.push(self.current_index);
        self.current_index += 1;
      } else {
        panic!("Couldn't insert {:?} {:#?}", position, self.cache);
      }
    } else if self.cache_impl == 2 {
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

fn non_precise_eq_vec3_f32([lx, ly, lz]: [f32; 3], [rx, ry, rz]: [f32; 3]) -> bool {
  non_precise_eq_f32(lx, rx) && non_precise_eq_f32(ly, ry) && non_precise_eq_f32(lz, rz)
}

fn non_precise_eq_vec2_f32([lx, ly]: [f32; 2], [rx, ry]: [f32; 2]) -> bool {
  non_precise_eq_f32(lx, rx) && non_precise_eq_f32(ly, ry)
}

fn non_precise_eq_f32(l: f32, r: f32) -> bool {
  (l * 1_000_000.0) as i32 == (r * 1_000_000.0) as i32
}

#[allow(clippy::many_single_char_names)]
fn subdivide(boundary: &Boundary, bucket: usize) -> Vec<MeshBuilderOctree> {
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

  let coords: [[f32; 3]; 8] = [
    [x - hw, y + hh, z + hd],
    [x + hw, y + hh, z + hd],
    [x - hw, y + hh, z - hd],
    [x + hw, y + hh, z - hd],
    [x - hw, y - hh, z + hd],
    [x + hw, y - hh, z + hd],
    [x - hw, y - hh, z - hd],
    [x + hw, y - hh, z - hd],
  ];

  let mut result = vec![];

  for coord in coords.iter() {
    result.push(MeshBuilderOctree::new(
      Boundary::new(*coord, size),
      bucket - 1,
    ));
  }

  result
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_octree_insert() {
    let mut tree = MeshBuilderOctree::new(Boundary::new([0.0, 0.0, 0.0], [16.0, 16.0, 16.0]), 10);

    assert!(tree.insert(&MeshBuilderData::new(
      [0.0, 0.0, 0.0],
      Some([0.0, 0.0, 0.0]),
      Some([0.0, 0.0]),
      0,
      0,
    )));

    assert_eq!(tree.get_all().len(), 1);
  }
}
