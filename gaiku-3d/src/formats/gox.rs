use gaiku_common::{prelude::*, Result};

use gox::{Block, Data, Gox, Only};

/// Converts a `gox` file to 3d chunk data.
pub struct GoxReader;

// TODO: The generated data appears rotated, need to rotate from back to bottom
impl FileFormat for GoxReader {
  type Value = (u8, u8);

  fn load<C, T>(bytes: Vec<u8>) -> Result<(Vec<C>, Option<TextureAtlas2d<T>>)>
  where
    C: Chunkify<Self::Value> + ChunkifyMut<Self::Value> + Boxify,
    T: Texturify2d,
  {
    let gox = Gox::from_bytes(bytes, vec![Only::Layers, Only::Blocks]);
    let mut colors: Vec<[u8; 4]> = Vec::with_capacity(255);
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
              let block_colors = block_data[data.block_index];
              let mut chunk = C::new([data.x as f32, data.y as f32, data.z as f32], 16, 16, 16);

              for x in 0..chunk.width() as usize {
                for y in 0..chunk.height() as usize {
                  for z in 0..chunk.depth() as usize {
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
                        chunk.set(x, y, z, (index as u8, 255));
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
    }

    if !colors.is_empty() {
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
