#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use mint::Vector2;

#[cfg(feature = "png")]
use std::{fs::File, io::BufWriter, path::Path};

use anyhow::Result;

pub const COLS: u32 = 16;
pub const ROWS: u32 = 16;
pub const COL_SIZE: f32 = 1.0 / COLS as f32;
pub const ROW_SIZE: f32 = 1.0 / COLS as f32;

fn index_to_xy(index: u8) -> (u8, u8) {
  (index % COLS as u8, index / COLS as u8)
}

fn xy_to_uv((x, y): (u8, u8)) -> (f32, f32) {
  (
    x as f32 / COLS as f32,
    (ROWS as u8 - 1 - y) as f32 / ROWS as f32,
  )
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextureAtlas2d {
  texture: Texture2d,
}

impl TextureAtlas2d {
  pub fn new(tile_size: u32) -> Self {
    Self::with_texture(Texture2d::new(COLS * tile_size, ROWS * tile_size))
  }

  pub fn with_texture(texture: Texture2d) -> Self {
    Self { texture }
  }

  pub fn get_texture(&self) -> Texture2d {
    self.texture.clone()
  }

  pub fn get_uv(&self, index: u8) -> (Vector2<f32>, Vector2<f32>, Vector2<f32>, Vector2<f32>) {
    let xy = index_to_xy(index);
    let (x, y) = xy_to_uv(xy);

    (
      [x, y].into(),
      [x + COL_SIZE, y].into(),
      [x + COL_SIZE, y + ROW_SIZE].into(),
      [x, y + ROW_SIZE].into(),
    )
  }

  pub fn set_at_index(&mut self, index: usize, pixels: Vec<[u8; 4]>) {
    if index + (pixels.len() * 4) < self.texture.data.len() {
      pixels
        .iter()
        .flat_map(|v| v.to_vec())
        .enumerate()
        .for_each(|(i, v)| self.texture.data[(index * 4) + i] = v);
    }
  }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Texture2d {
  width: u32,
  height: u32,
  data: Vec<u8>,
}

impl Texture2d {
  pub fn new(width: u32, height: u32) -> Self {
    Self {
      width,
      height,
      data: vec![0; (width as usize * height as usize) * 4],
    }
  }

  pub fn get_data(&self) -> Vec<u8> {
    self.data.clone()
  }

  pub fn get_pixel(&self, x: u32, y: u32) -> Option<[u8; 4]> {
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

  pub fn height(&self) -> u32 {
    self.height
  }

  pub fn width(&self) -> u32 {
    self.width
  }

  pub fn set_pixel(&mut self, x: u32, y: u32, data: [u8; 4]) {
    if x < self.width && y < self.height {
      let index = (x * 4 + self.width * y * 4) as usize;

      for (i, value) in data.iter().enumerate() {
        self.data[index + i] = *value;
      }
    }
  }

  #[cfg(feature = "png")]
  pub fn write_to_file(&self, file_path: &str) -> Result<()> {
    let path = Path::new(file_path);
    let file = File::create(path)?;
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, self.width, self.height);
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let writer = &mut encoder.write_header()?;

    writer.write_image_data(&self.data)?;

    Ok(())
  }
}

#[cfg(test)]
mod test {
  use super::*;

  fn get_uv_helper(
    atlas: &TextureAtlas2d,
    x: u32,
    y: u32,
  ) -> ([f32; 2], [f32; 2], [f32; 2], [f32; 2]) {
    let index = x + y * ROWS;
    assert!(index <= 255);
    let uv = atlas.get_uv(index as u8);
    (uv.0.into(), uv.1.into(), uv.2.into(), uv.3.into())
  }

  #[test]
  fn test_index_to_xy() {
    assert_eq!((0, 1), index_to_xy(0 + 1 * COLS as u8));
    assert_eq!((1, 0), index_to_xy(1 + 0 * COLS as u8));
    assert_eq!((15, 1), index_to_xy(15 + 1 * COLS as u8));
    assert_eq!((12, 15), index_to_xy(12 + 15 * COLS as u8));
    assert_eq!((15, 15), index_to_xy(15 + 15 * COLS as u8));
  }

  #[test]
  fn test_texture_size() {
    let tile_size = 16;
    let atlas = TextureAtlas2d::new(tile_size);

    assert_eq!(256, atlas.texture.width);
    assert_eq!(256, atlas.texture.height);
  }

  #[test]
  fn test_texture_atlas_index_to_uv() {
    let tile_size = 1;
    let atlas = TextureAtlas2d::new(tile_size);

    let uv = get_uv_helper(&atlas, 0, 0);
    assert_eq!(uv.0, [0.0000, 0.0000]);
    assert_eq!(uv.1, [0.0625, 0.0000]);
    assert_eq!(uv.2, [0.0625, 0.0625]);
    assert_eq!(uv.3, [0.0000, 0.0625]);

    let uv = get_uv_helper(&atlas, 15, 0);
    assert_eq!(uv.0, [0.9375, 0.0000]);
    assert_eq!(uv.1, [0.9375, 0.0000]);
    assert_eq!(uv.2, [0.9375, 0.0000]);
    assert_eq!(uv.3, [0.9375, 0.0000]);

    let uv = get_uv_helper(&atlas, 1, 0);
    assert_eq!(uv.0, [0.0625, 0.0000]);
    assert_eq!(uv.1, [0.0625, 0.0000]);
    assert_eq!(uv.2, [0.0625, 0.0000]);
    assert_eq!(uv.3, [0.0625, 0.0000]);

    let uv = get_uv_helper(&atlas, 15, 15);
    assert_eq!(uv.0, [0.9375, 0.9375]);
    assert_eq!(uv.1, [0.9375, 0.9375]);
    assert_eq!(uv.2, [0.9375, 0.9375]);
    assert_eq!(uv.3, [0.9375, 0.9375]);
  }
}
