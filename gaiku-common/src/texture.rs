#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "png")]
use std::{fs::File, io::BufWriter, path::Path};

use anyhow::Result;

pub(crate) const COLS: u32 = 16;
pub(crate) const ROWS: u32 = 16;
pub(crate) const COL_SIZE: f32 = 1.0 / COLS as f32;
pub(crate) const ROW_SIZE: f32 = 1.0 / COLS as f32;
pub(crate) const COL_PADDING: f32 = COL_SIZE * 1e-5;
pub(crate) const ROW_PADDING: f32 = ROW_SIZE * 1e-5;

fn index_to_xy(index: u8) -> (u8, u8) {
  (index % COLS as u8, index / COLS as u8)
}

fn xy_to_uv((x, y): (u8, u8)) -> (f32, f32) {
  (
    x as f32 / COLS as f32,
    (ROWS as u8 - 1 - y) as f32 / ROWS as f32,
  )
}

/// Base common denominator across all the 2d texture implementations used.
pub trait Texturify2d: Clone + std::fmt::Debug {
  fn new(width: u32, height: u32) -> Self;
  fn get_data(&self) -> &Vec<u8>;
  fn get_pixel(&self, x: u32, y: u32) -> Option<[u8; 4]>;
  fn height(&self) -> u32;
  fn len(&self) -> usize;
  fn set_pixel(&mut self, x: u32, y: u32, data: [u8; 4]);
  fn set_pixel_at_index(&mut self, index: usize, data: [u8; 4]);
  fn width(&self) -> u32;
}

/// A convenience component to work with tiled textures.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TextureAtlas2d<T>
where
  T: Texturify2d,
{
  texture: T,
}

impl<T> TextureAtlas2d<T>
where
  T: Texturify2d,
{
  pub fn new(tile_size: u32) -> Self {
    Self::with_texture(T::new(COLS * tile_size, ROWS * tile_size))
  }

  pub fn with_texture(texture: T) -> Self {
    Self { texture }
  }

  pub fn get_texture(&self) -> T {
    self.texture.clone()
  }

  pub fn get_uv(&self, index: u8) -> ([f32; 2], [f32; 2], [f32; 2], [f32; 2]) {
    let xy = index_to_xy(index);
    let (x, y) = xy_to_uv(xy);
    // add padding between the tile borders and the uv
    (
      [x + COL_PADDING, y + ROW_PADDING],
      [x + COL_SIZE - COL_PADDING, y + ROW_PADDING],
      [x + COL_SIZE - COL_PADDING, y + ROW_SIZE - ROW_PADDING],
      [x + COL_PADDING, y + ROW_SIZE - ROW_PADDING],
    )
  }

  pub fn set_at_index(&mut self, index: u8, pixels: Vec<[u8; 4]>) {
    // Get UV position on the tex for this index
    let xy = index_to_xy(index);
    let uv = xy_to_uv(xy);

    // Convert uv to tex xy for the origin of this blit
    let x_o = (uv.0 * self.texture.width() as f32).floor() as u32; // Convert uv to tex xy
    let y_o = (uv.1 * self.texture.height() as f32).floor() as u32; // Convert uv to tex xy

    // Get the width of the tile in texture coords so we can blit that area with the pixels
    let tile_width = (COL_SIZE * self.texture.width() as f32).floor() as u32;

    // Blit the texture's tile
    pixels.iter().enumerate().for_each(|(i, v)| {
      let (dx, dy) = (i as u32 % tile_width, i as u32 / tile_width);
      let (x, y) = (x_o + dx, y_o + dy);
      self.texture.set_pixel(x, y, *v);
    });
  }
}

/// Provides a `Texturify2d` implementation based on an RGBA values.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Texture2d {
  width: u32,
  height: u32,
  data: Vec<u8>,
}

impl Texture2d {
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

impl Texturify2d for Texture2d {
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

#[cfg(test)]
mod test {
  use super::*;
  use std::convert::TryInto;

  fn get_uv_helper(
    atlas: &TextureAtlas2d<Texture2d>,
    x: u32,
    y: u32,
  ) -> ([f32; 2], [f32; 2], [f32; 2], [f32; 2]) {
    let index = x + y * ROWS;
    assert!(index <= 255);
    let uv = atlas.get_uv(index as u8);
    (uv.0, uv.1, uv.2, uv.3)
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
    let atlas = TextureAtlas2d::<Texture2d>::new(tile_size);

    assert_eq!(256, atlas.texture.width);
    assert_eq!(256, atlas.texture.height);
  }

  #[test]
  fn test_texture_atlas_index_to_uv() {
    let tile_size = 1;
    let atlas = TextureAtlas2d::<Texture2d>::new(tile_size);

    let uv = get_uv_helper(&atlas, 0, 0);
    assert_eq!(uv.0, [0.0000, 0.9375]);
    assert_eq!(uv.1, [0.0625, 0.9375]);
    assert_eq!(uv.2, [0.0625, 1.0]);
    assert_eq!(uv.3, [0.0000, 1.0]);

    let uv = get_uv_helper(&atlas, 15, 0);
    assert_eq!(uv.0, [0.9375, 0.9375]);
    assert_eq!(uv.1, [1.0000, 0.9375]);
    assert_eq!(uv.2, [1.0000, 1.0000]);
    assert_eq!(uv.3, [0.9375, 1.0000]);

    let uv = get_uv_helper(&atlas, 1, 0);
    assert_eq!(uv.0, [0.0625, 0.9375]);
    assert_eq!(uv.1, [0.1250, 0.9375]);
    assert_eq!(uv.2, [0.1250, 1.0000]);
    assert_eq!(uv.3, [0.0625, 1.0000]);

    let uv = get_uv_helper(&atlas, 15, 15);
    assert_eq!(uv.0, [0.9375, 0.0000]);
    assert_eq!(uv.1, [1.0000, 0.0000]);
    assert_eq!(uv.2, [1.0000, 0.0625]);
    assert_eq!(uv.3, [0.9375, 0.0625]);
  }

  #[test]
  fn test_texture_atlas_created_tex_size() {
    let tile_size = 3;
    let atlas = TextureAtlas2d::<Texture2d>::new(tile_size);
    let data_size = atlas.texture.data.len();
    assert_eq!(
      data_size,
      (tile_size * COLS * tile_size * ROWS * 4)
        .try_into()
        .unwrap()
    );

    let tile_size = 2;
    let atlas = TextureAtlas2d::<Texture2d>::new(tile_size);
    let data_size = atlas.texture.get_data().len();
    assert_eq!(
      data_size,
      (tile_size * COLS * tile_size * ROWS * 4)
        .try_into()
        .unwrap()
    );
  }

  #[test]
  fn test_texture_atlas_pixel_set() {
    let tile_size = 3;
    let index = 1;

    let mut atlas = TextureAtlas2d::<Texture2d>::new(tile_size);

    let test_pixels: [[u8; 4]; 9] = [
      [10, 20, 30, 40], // Row 1
      [11, 21, 31, 41],
      [12, 22, 32, 42],
      [110, 120, 130, 140], // Row 2
      [111, 121, 131, 141],
      [112, 122, 132, 142],
      [210, 220, 230, 240], // Row 3
      [211, 221, 231, 241],
      [212, 222, 232, 242],
    ];

    atlas.set_at_index(index, test_pixels.to_vec());
    let tex = atlas.get_texture();
    let tex_data = tex.get_data();

    // Convert some things to usize now to avoid multiple casts in the rest of the test
    let tile_size: usize = tile_size.try_into().unwrap();
    let cols: usize = COLS.try_into().unwrap();
    let rows: usize = ROWS.try_into().unwrap();

    // Make bytes for comparison manually that should match those generated by the atlas
    let mut test_data: Vec<u8> = vec![];
    // Fill with zeros until row that first tile starts
    test_data.append(
      &mut std::iter::repeat(0)
        .take(tile_size * cols * (rows - 1) * tile_size * 4)
        .collect::<Vec<_>>(),
    );
    // Fill with zeros until the first tile
    test_data.append(&mut std::iter::repeat(0).take(tile_size * 4).collect::<Vec<_>>());
    // Append first row of the tile
    test_data.append(&mut test_pixels[0].to_vec());
    test_data.append(&mut test_pixels[1].to_vec());
    test_data.append(&mut test_pixels[2].to_vec());
    // Fill with zeros until the next row
    test_data.append(
      &mut std::iter::repeat(0)
        .take(tile_size * (cols - 1) * 4)
        .collect::<Vec<_>>(),
    );
    // Append the second row of pixels to the tile
    test_data.append(&mut test_pixels[3].to_vec());
    test_data.append(&mut test_pixels[4].to_vec());
    test_data.append(&mut test_pixels[5].to_vec());
    // Fill with zeros until the next row
    test_data.append(
      &mut std::iter::repeat(0)
        .take(tile_size * (cols - 1) * 4)
        .collect::<Vec<_>>(),
    );
    // Append the third row of pixels to the tile
    test_data.append(&mut test_pixels[6].to_vec());
    test_data.append(&mut test_pixels[7].to_vec());
    test_data.append(&mut test_pixels[8].to_vec());
    // pad remaining bytes with zeros
    test_data.append(
      &mut std::iter::repeat(0)
        .take(tile_size * (cols - 2) * 4)
        .collect::<Vec<_>>(),
    );

    // Test equality between texture made from the atlas and that made manually here
    assert_eq!(tex_data, &test_data);
  }
}
