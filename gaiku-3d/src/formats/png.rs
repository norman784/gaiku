use gaiku_common::{prelude::*, Result};

use image::load_from_memory;

/// Converts a `png` file to 2d chunk data.
#[derive(Default)]
pub struct PNGReader;

impl FileFormat<(u8, u8)> for PNGReader {
  fn load<C, T>(bytes: Vec<u8>) -> Result<(Vec<C>, Option<TextureAtlas2d<T>>)>
  where
    C: Chunkify<(u8, u8)> + Boxify,
    T: Texturify2d,
  {
    let mut result = vec![];
    let img = load_from_memory(&bytes)?.into_luma8();

    assert!(img.width() <= u16::MAX as u32);
    assert!(img.height() <= u16::MAX as u32);

    let mut chunk = C::new([0.0, 0.0, 0.0], img.width() as u16, img.height() as u16, 1);

    for x in 0..img.width() as u32 {
      for y in 0..img.height() as u32 {
        let color = img.get_pixel(x, y).0[0];
        chunk.set(x as usize, y as usize, 0, (color, color));
      }
    }

    result.push(chunk);

    Ok((result, None))
  }
}
