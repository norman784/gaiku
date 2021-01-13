use gaiku_common::{mesh::Meshify, texture::Texturify2d};

use amethyst::renderer::{
  palette::Srgba,
  rendy::{
    hal::{
      image::{Filter, Kind, SamplerInfo, ViewKind, WrapMode},
      Primitive,
    },
    mesh::{MeshBuilder, Normal, Position, TexCoord},
    texture::{pixel::Rgba8Srgb, TextureBuilder},
  },
  types::{MeshData, TextureData},
};

pub mod prelude {
  pub use crate::{GaikuMesh, GaikuTexture2d};
  pub use gaiku_common::prelude::*;
}

pub struct GaikuMesh {
  pub indices: Vec<u32>,
  pub positions: Vec<[f32; 3]>,
  pub normals: Vec<[f32; 3]>,
  pub uvs: Vec<[f32; 2]>,
}

impl Meshify for GaikuMesh {
  fn new() -> Self {
    Self::with(vec![], vec![], vec![], vec![])
  }

  fn with(
    indices: Vec<u32>,
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
  ) -> Self {
    Self {
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
}

impl Into<MeshData> for GaikuMesh {
  fn into(self) -> MeshData {
    let ame = MeshBuilder::new()
      .with_vertices(
        self
          .positions
          .iter()
          .map(|v| (*v).into())
          .collect::<Vec<Position>>(),
      )
      .with_vertices(
        self
          .uvs
          .iter()
          .map(|v| (*v).into())
          .collect::<Vec<TexCoord>>(),
      )
      .with_vertices(
        self
          .normals
          .iter()
          .map(|v| (*v).into())
          .collect::<Vec<Normal>>(),
      )
      .with_indices(self.indices.clone())
      .with_prim_type(Primitive::TriangleList);

    ame.into()
  }
}

#[derive(Debug, Clone)]
pub struct GaikuTexture2d {
  width: u32,
  height: u32,
  data: Vec<u8>,
}

impl Texturify2d for GaikuTexture2d {
  fn new(width: u32, height: u32) -> Self {
    Self {
      width,
      height,
      data: vec![0; (width as usize * height as usize) * 4],
    }
  }

  fn get_data(&self) -> &Vec<u8> {
    &self.data
  }

  fn get_pixel(&self, x: u32, y: u32) -> Option<[u8; 4]> {
    if x < self.width && y < self.height {
      let index = (x + 4 + self.width * y + 4) as usize;
      Some([
        self.data[index],
        self.data[index + 1],
        self.data[index + 2],
        self.data[index + 3],
      ])
    } else {
      None
    }
  }

  fn height(&self) -> u32 {
    self.height
  }

  fn len(&self) -> usize {
    self.data.len()
  }

  fn set_pixel(&mut self, x: u32, y: u32, data: [u8; 4]) {
    if x < self.width && y < self.height {
      self.set_pixel_at_index((x * 4 + self.width * y * 4) as usize, data);
    }
  }

  fn set_pixel_at_index(&mut self, index: usize, data: [u8; 4]) {
    if index < self.data.len() - 4 {
      for (i, value) in data.iter().enumerate() {
        self.data[index + i] = *value;
      }
    }
  }

  fn width(&self) -> u32 {
    self.width
  }
}

impl Into<TextureData> for GaikuTexture2d {
  fn into(self) -> TextureData {
    let texture_builder = TextureBuilder::new()
      .with_kind(Kind::D2(self.width, self.height, 1, 1))
      .with_view_kind(ViewKind::D2)
      .with_data_width(self.width)
      .with_data_height(self.height)
      .with_sampler_info(SamplerInfo::new(Filter::Linear, WrapMode::Clamp))
      .with_data(
        self
          .data
          .chunks(4)
          .map(|color| Rgba8Srgb::from(Srgba::new(color[0], color[1], color[2], color[3])))
          .collect::<Vec<_>>(),
      );
    texture_builder.into()
  }
}
