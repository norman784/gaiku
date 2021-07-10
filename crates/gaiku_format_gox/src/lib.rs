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
    let gox = Gox::from_bytes(bytes, vec![Only::Layers, Only::Blocks]);
    let mut colors: Vec<[u8; 4]> = Vec::with_capacity(255);
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
    let gox_chunk_size = [16, 16, 16];

    let min = [
      starts.iter().map(|v| v[0]).min().unwrap(),
      starts.iter().map(|v| v[1]).min().unwrap(),
      starts.iter().map(|v| v[2]).min().unwrap(),
    ];
    let max = [
      starts.iter().map(|v| v[0]).max().unwrap() + gox_chunk_size[0],
      starts.iter().map(|v| v[1]).max().unwrap() + gox_chunk_size[1],
      starts.iter().map(|v| v[2]).max().unwrap() + gox_chunk_size[2],
    ];

    let data_dim: [usize; 3] = [
      (max[0] - min[0] + 1).try_into().unwrap(),
      (max[1] - min[1] + 1).try_into().unwrap(),
      (max[2] - min[2] + 1).try_into().unwrap(),
    ];
    let data_size = data_dim[0] * data_dim[1] * data_dim[2];

    let mut values = vec![-1.; data_size];
    let mut atlas_values = vec![0; data_size];

    for data in gox.data.iter() {
      if let Data::Layers(layers, _bounds) = &data {
        for layer in layers.iter() {
          if !layer.blocks.is_empty() {
            for data in layer.blocks.iter() {
              let block_colors = block_data[data.block_index];
              let origin: [usize; 3] = [
                (data.x - min[0]).try_into().unwrap(),
                (data.y - min[1]).try_into().unwrap(),
                (data.z - min[2]).try_into().unwrap(),
              ];

              // println!("====");
              // println!("Origin: {:?}", origin);

              for x in 0..16 {
                let x_c = x + origin[0];
                for y in 0..16 {
                  let y_c = y + origin[1];
                  for z in 0..16 {
                    let z_c = z + origin[2];
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
                        // Swap y and z axis
                        // Gox is in z up
                        // Gaiku in y up
                        let idx = x_c + z_c * data_dim[0] + y_c * data_dim[2] * data_dim[0];
                        values[idx] = 1.;
                        atlas_values[idx] = index as Self::AtlasValue
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

    let mut chunker: FlatChunker<C> = FlatChunker::from_array_with_atlas(
      values.as_slice(),
      atlas_values.as_slice(),
      data_dim[0],
      data_dim[2], // Swap y and z axis
      data_dim[1], // Gox is in z up
                   // Gaiku in y up
    );
    chunker.set_chunk_size([16, 16, 16]);
    chunker.generate_chunks();
    let mut chunked = chunker.get_chunks_mut();

    // Because we don't assume clone we take ownership by swapping with some default
    // we can do this because we don't want to use chunker of chunked again
    let result: Vec<C> = chunked
      .iter_mut()
      .map(|c| std::mem::replace(&mut c.chunk, C::new([0., 0., 0.], 0, 0, 0)))
      .collect();

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
