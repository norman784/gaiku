use gaiku_common::{Chunk, Chunkify, FileFormat, TextureAtlas2d, Result};

use gox::{Block, Data, Gox, Only};

use std::fs::File;

pub struct GoxReader;

// TODO: The generated data appears rotated, need to rotate from back to bottom
impl FileFormat for GoxReader {
  fn load(stream: &mut File) -> Result<(Vec<Chunk>, Option<TextureAtlas2d>)> {
    let gox = Gox::new(stream, vec![Only::Layers, Only::Blocks]);
    let mut colors: Vec<[u8; 4]> = Vec::with_capacity(255);
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
                let block_colors = block_data[data.block_index];
                let mut chunk =
                  Chunk::new([data.x as f32, data.y as f32, data.z as f32], 16, 16, 16);

                for x in 0..chunk.width() {
                  for y in 0..chunk.height() {
                    for z in 0..chunk.depth() {
                      if !block_colors.is_empty(x, y, z) {
                        let color = block_colors.get_pixel(x, y, z);
                        let index = if let Some((index, _)) =
                          colors.iter().enumerate().find(|(_, value)| {
                            value[0] == color[0]
                              && value[1] == color[1]
                              && value[2] == color[2]
                              && value[3] == color[3]
                          }) {
                          index
                        } else {
                          let index = colors.len();
                          colors.push(color);
                          index
                        };

                        if index <= std::u8::MAX as usize {
                          chunk.set(x, y, z, index as u8);
                        }
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

    if colors.len() > 0 {
      let mut atlas = TextureAtlas2d::new(1);

      for (index, color) in colors.iter().enumerate() {
        atlas.set_at_index(index, vec![*color]);
      }

      Ok((result, Some(atlas)))
    } else {
      Ok((result, None))
    }
  }
}
