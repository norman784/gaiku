use gaiku_common::{Chunk, Chunkify, FileFormat, Result, TextureAtlas2d};

use png::{ColorType, Decoder};

pub struct PNGReader;

impl FileFormat for PNGReader {
  fn load(bytes: Vec<u8>) -> Result<(Vec<Chunk>, Option<TextureAtlas2d>)> {
    let mut result = vec![];
    let decoder = Decoder::new(stream);

    let (info, mut reader) = decoder.read_info()?;
    let mut buf = vec![0; info.buffer_size()];

    reader.next_frame(&mut buf)?;

    let data = match info.color_type {
      ColorType::RGB => buf,
      ColorType::RGBA => buf,
      ColorType::Grayscale => {
        let mut vec = Vec::with_capacity(buf.len() * 3);
        for g in buf {
          vec.extend([g, g, g].iter().cloned())
        }
        vec
      }
      ColorType::GrayscaleAlpha => {
        let mut vec = Vec::with_capacity(buf.len() * 3);
        for ga in buf.chunks(2) {
          let g = ga[0];
          let a = ga[1];
          vec.extend([g, g, g, a].iter().cloned())
        }
        vec
      }
      _ => unreachable!("uncovered color type"),
    };

    let mut colors = vec![[0; 4]; (info.width * info.height) as usize];
    for (i, color) in data.chunks(4).enumerate() {
      if color.len() == 3 {
        colors[i] = [color[0], color[1], color[2], 255];
      } else {
        colors[i] = [color[0], color[1], color[2], color[3]];
      }
    }

    let mut chunk = Chunk::new(
      [0.0, 0.0, 0.0],
      info.width as usize,
      info.height as usize,
      1,
    );

    for x in 0..info.width as usize {
      for y in 0..info.height as usize {
        let color = colors[x + y * info.width as usize];
        let value = color[0] | color[1];
        chunk.set(x, y, 0, (value, value));
      }
    }

    result.push(chunk);

    Ok((result, None))
  }
}
