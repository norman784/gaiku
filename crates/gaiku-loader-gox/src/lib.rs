use anyhow::Result;
use gaiku_common::{Chunk, FileFormat};
use gox::{self, Block, Data, Only};

pub struct Gox;

// TODO: The generated data appears rotated, need to rotate from back to bottom
impl FileFormat for Gox {
  fn from_bytes(bytes: Vec<u8>) -> Result<Vec<Chunk>> {
    let gox = gox::Gox::from_bytes(bytes, vec![Only::Layers, Only::Blocks]);
    let mut result = vec![];
    let mut block_data: Vec<&Block> = vec![];

    for data in gox.data.iter() {
      match &data {
        Data::Blocks(data) => {
          block_data.push(data);
        }
        _ => {}
      }
    }

    for data in gox.data.iter() {
      match &data {
        Data::Layers(layers, _bounds) => {
          for layer in layers.iter() {
            if layer.blocks.len() > 0 {
              for data in layer.blocks.iter() {
                let colors = block_data[data.block_index];
                let mut chunk =
                  Chunk::new([data.x as f32, data.y as f32, data.z as f32], 16, 16, 16);

                for x in 0..chunk.width() {
                  for y in 0..chunk.height() {
                    for z in 0..chunk.depth() {
                      if !colors.is_empty(x, y, z) {
                        chunk.set(x, y, z, 255);
                        chunk.set_color(x, y, z, colors.get_pixel(x, y, z));
                      }
                    }
                  }
                }

                result.push(chunk);
              }
            }
          }
        }
        _ => {}
      }
    }

    Ok(result)
  }
}
