use gaiku_common::{Chunk, Chunkify, FileFormat, Result, TextureAtlas2d};

use image::load_from_memory;

pub struct PNGReader;

impl FileFormat for PNGReader {
  fn load(bytes: Vec<u8>) -> Result<(Vec<Chunk>, Option<TextureAtlas2d>)> {
    let mut result = vec![];
    let img = load_from_memory(&bytes)?.into_luma8();

    let mut chunk = Chunk::new(
      [0.0, 0.0, 0.0],
      img.width().clamp(0, u16::MAX as u32) as u16,
      img.height().clamp(0, u16::MAX as u32) as u16,
      1,
    );

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
