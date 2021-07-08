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
