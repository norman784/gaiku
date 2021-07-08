use gaiku_common::{prelude::*, Result};
use std::convert::TryInto;

use gox::{Block, Data, Gox, Only};

/// Converts a `gox` file to 3d chunk data.
pub struct GoxReader;

// TODO: The generated data appears rotated, need to rotate from back to bottom
impl FileFormat for GoxReader {
  type Value = f32;
  type AtlasValue = u8;

  fn load<C, T>(bytes: Vec<u8>) -> Result<(Vec<C>, Option<TextureAtlas2d<T>>)>
  where
    C: Chunkify<Self::Value> + ChunkifyMut<Self::Value> + AtlasifyMut<Self::AtlasValue> + Boxify,
    T: Texturify2d,
  {
    type Coord = usize;
    let gox = Gox::from_bytes(bytes, vec![Only::Layers, Only::Blocks]);
    let mut colors: Vec<[u8; 4]> = Vec::with_capacity(255);
    let mut result = vec![];
    let mut block_data: Vec<&Block> = vec![];

    for data in gox.data.iter() {
      if let Data::Blocks(data) = &data {
        block_data.push(data);
      }
    }

    let mut starts = vec![];
    for data in gox.data.iter() {
      if let Data::Layers(layers, _bounds) = &data {
        for layer in layers.iter() {
          if !layer.blocks.is_empty() {
            for data in layer.blocks.iter() {
              starts.push([data.x, data.y, data.z]);
            }
          }
        }
      }
    }
    let init_coord = starts[0];

    let min = [
      starts
        .iter()
        .fold(init_coord[0], |acc, c| if c[0] < acc { c[0] } else { acc }),
      starts
        .iter()
        .fold(init_coord[1], |acc, c| if c[1] < acc { c[1] } else { acc }),
      starts
        .iter()
        .fold(init_coord[2], |acc, c| if c[2] < acc { c[2] } else { acc }),
    ];
    let max = [
      starts.iter().fold(init_coord[0] + 16, |acc, c| {
        if c[0] + 16 > acc {
          c[0] + 16
        } else {
          acc
        }
      }),
      starts.iter().fold(init_coord[1] + 16, |acc, c| {
        if c[1] + 16 > acc {
          c[1] + 16
        } else {
          acc
        }
      }),
      starts.iter().fold(init_coord[2] + 16, |acc, c| {
        if c[2] + 16 > acc {
          c[2] + 16
        } else {
          acc
        }
      }),
    ];

    let chunk_size: [Coord; 3] = [
      (max[0] - min[0] + 1).try_into().unwrap(),
      (max[1] - min[1] + 1).try_into().unwrap(),
      (max[2] - min[2] + 1).try_into().unwrap(),
    ];

    let mut chunk = C::new(
      [(min[0]) as f32, (min[2]) as f32, (min[1]) as f32],
      chunk_size[0].try_into().unwrap(),
      chunk_size[2].try_into().unwrap(), // goxel is in y up gaiku in z up
      chunk_size[1].try_into().unwrap(),
    );

    for data in gox.data.iter() {
      if let Data::Layers(layers, _bounds) = &data {
        for layer in layers.iter() {
          if !layer.blocks.is_empty() {
            for data in layer.blocks.iter() {
              let block_colors = block_data[data.block_index];
              let origin: [Coord; 3] = [
                (data.x - min[0]).try_into().unwrap(),
                (data.y - min[1]).try_into().unwrap(),
                (data.z - min[2]).try_into().unwrap(),
              ];

              for x in 0..16 {
                let x_c = x as Coord + origin[0];
                for y in 0..16 {
                  let y_c = y as Coord + origin[1];
                  for z in 0..16 {
                    let z_c = z as Coord + origin[2];
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
                        chunk.set(x_c, z_c, y_c, 1.); // goxel is in y up gaiku in zup
                        chunk.set_atlas(x_c, z_c, y_c, index as Self::AtlasValue);
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }

    result.push(chunk);

    if !colors.is_empty() {
      let mut atlas = TextureAtlas2d::new(1);

      for (index, color) in colors.iter().enumerate() {
        // colors should limited to 255 so (index.try_into().unwrap()) should fit into u8 for set_at_index
        atlas.fill_at_index(index.try_into().unwrap(), *color);
      }

      Ok((result, Some(atlas)))
    } else {
      Ok((result, None))
    }
  }
}
