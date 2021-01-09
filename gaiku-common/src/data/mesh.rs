use std::{borrow::Cow, collections::HashMap};

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

#[derive(Debug)]
struct MeshBuilderData {
  position: [f32; 3],
  normal: Option<[f32; 3]>,
  uv: Option<[f32; 2]>,
  atlas_index: u16,
  index: u32,
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
    MeshBuilderData {
      position,
      normal,
      uv,
      atlas_index,
      index,
    }
  }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Position(i32, i32, i32);

impl From<[f32; 3]> for Position {
  fn from([x, y, z]: [f32; 3]) -> Self {
    Position(
      (x * 1_000_0.0) as i32,
      (y * 1_000_0.0) as i32,
      (z * 1_000_0.0) as i32,
    )
  }
}

#[derive(Debug)]
pub struct MeshBuilder {
  indices: Vec<u32>,
  cache: HashMap<Position, Vec<MeshBuilderData>>,
}

impl MeshBuilder {
  pub fn create() -> Self {
    Self {
      indices: vec![],
      cache: HashMap::new(),
    }
  }

  pub fn add(
    &mut self,
    position: [f32; 3],
    normal: Option<[f32; 3]>,
    uv: Option<[f32; 2]>,
    atlas_index: u16,
  ) {
    let next_index = self.cache.values().fold(0, |acc, v| acc + v.len()) as u32;
    let data = self.cache.entry(position.into()).or_insert_with(Vec::new);

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

    data.push(MeshBuilderData {
      position,
      normal,
      uv,
      atlas_index,
      index: next_index,
    });

    self.indices.push(next_index);
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
    None
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
