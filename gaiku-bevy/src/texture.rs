use bevy_render::texture::{Extent3d, Texture, TextureDimension, TextureFormat};
use gaiku_common::prelude::*;

#[derive(Clone, Debug)]
pub struct GaikuTexture {
  data: Vec<u8>,
  height: u32,
  width: u32,
}

impl GaikuTexture {
  fn index(&self, x: u32, y: u32) -> usize {
    (x + y * self.width) as usize
  }
}

impl Texturify2d for GaikuTexture {
  fn new(width: u32, height: u32) -> Self {
    Self {
      data: vec![0; (width * height * 4) as usize],
      height,
      width,
    }
  }

  fn get_data(&self) -> &Vec<u8> {
    &self.data
  }

  fn get_pixel(&self, x: u32, y: u32) -> Option<[u8; 4]> {
    let index = self.index(x, y);

    if index < self.data.len() - 4 {
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
    let index = self.index(x, y);
    self.set_pixel_at_index(index, data);
  }

  fn set_pixel_at_index(&mut self, index: usize, data: [u8; 4]) {
    if index < self.data.len() - 4 {
      data
        .iter()
        .enumerate()
        .for_each(|(i, v)| self.data[index + i] = *v);
    }
  }

  fn width(&self) -> u32 {
    self.width
  }
}

impl Into<Texture> for GaikuTexture {
  fn into(self) -> Texture {
    Texture::new(
      Extent3d::new(self.width, self.height, 1),
      TextureDimension::D2,
      self.data.clone(),
      TextureFormat::Rgba8Uint,
    )
  }
}
