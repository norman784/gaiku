use gaiku_common::{Chunk, Chunkify, FileFormat, Result, TextureAtlas2d};

use image::load_from_memory;

pub struct PNGReader;

impl FileFormat for PNGReader {
  fn load(bytes: Vec<u8>) -> Result<(Vec<Chunk>, Option<TextureAtlas2d>)> {
    let mut result = vec![];
    let img = load_from_memory(&bytes)?.into_luma8();

    let mut chunk = Chunk::new(
      [0.0, 0.0, 0.0],
      img.width() as usize,
      img.height() as usize,
      1,
    );

    for x in 0..img.width() {
      for y in 0..img.height() {
        let color = img.get_pixel(x, y).0[0];
        chunk.set(x as usize, y as usize, 0, (color, color));
      }
    }

    result.push(chunk);

    Ok((result, None))
  }
}
