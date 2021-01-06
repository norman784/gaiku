use gaiku_common::{Chunk, Chunkify, FileFormat};

use gox::{Block, Data, Gox, Only};

use std::fs::File;

pub struct GoxReader;

// TODO: The generated data appears rotated, need to rotate from back to bottom
impl FileFormat for GoxReader {
  fn load(stream: &mut File) -> Vec<Chunk> {
    let gox = Gox::new(stream, vec![Only::Layers, Only::Blocks]);
    let mut result = vec![];
    let mut block_data: Vec<&Block> = vec![];

    for data in gox.data.iter() {
      if let Data::Blocks(data) = &data {
        block_data.push(data);
      }
    }

    for data in gox.data.iter() {
      if let Data::Layers(layers, _bounds) = &data {
        for layer in layers.iter() {
          if !layer.blocks.is_empty() {
            for data in layer.blocks.iter() {
              let colors = block_data[data.block_index];
              let mut chunk = Chunk::new([data.x as f32, data.y as f32, data.z as f32], 16, 16, 16);

              for x in 0..chunk.width() {
                for y in 0..chunk.height() {
                  for z in 0..chunk.depth() {
                    if !colors.is_empty(x, y, z) {
                      chunk.set_with_color(x, y, z, 255, colors.get_pixel(x, y, z).into());
                    }
                  }
                }
              }

              result.push(chunk);
            }
          }
        }
      }
    }

    result
  }
}
