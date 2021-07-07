// Takes input data and generates a
// series of chunks at specified size
use super::common::*;
use crate::{
  boxify::Sizable,
  chunk::{Chunkify, ChunkifyMut},
};
use std::convert::TryInto;

#[derive(Clone)]
pub struct FlatChunker {
  data: Vec<f32>,
  data_width: usize,
  data_height: usize,
  data_depth: usize,
  chunk_sizes: [u16; 3],
}

impl<C> Chunker<C, f32> for FlatChunker
where
  C: Chunkify<f32> + ChunkifyMut<f32> + Sizable,
{
  fn from_array(data: &[f32], width: usize, height: usize, depth: usize) -> Self {
    Self {
      data: data.to_vec(),
      data_width: width,
      data_height: height,
      data_depth: depth,
      chunk_sizes: [16, 16, 16],
    }
  }

  fn generate_chunks(&self) -> Vec<Chunked<C>> {
    let mut results = vec![];
    let chunk_sizes = [
      self.chunk_sizes[0] as usize,
      self.chunk_sizes[1] as usize,
      self.chunk_sizes[2] as usize,
    ];
    for x in 0..(self.data_width / chunk_sizes[0] + 1) {
      for y in 0..(self.data_height / chunk_sizes[1] + 1) {
        for z in 0..(self.data_depth / chunk_sizes[2] + 1) {
          let x_min = x * self.data_width;
          let x_max = std::cmp::min(x_min + chunk_sizes[0] + 1, self.data_width);
          let x_size = x_max - x_min;
          if x_size == 0 {
            continue;
          }

          let y_min = y * self.data_height;
          let y_max = std::cmp::min(y_min + chunk_sizes[1] + 1, self.data_height);
          let y_size = y_max - y_min;
          if y_size == 0 {
            continue;
          }

          let z_min = z * self.data_height;
          let z_max = std::cmp::min(z_min + chunk_sizes[2] + 1, self.data_height);
          let z_size = z_max - z_min;
          if z_size == 0 {
            continue;
          }

          let mut chunk = C::with_size(
            x_size.try_into().unwrap(),
            y_size.try_into().unwrap(),
            z_size.try_into().unwrap(),
          );

          for i in x_min..x_max {
            for j in y_min..y_max {
              for k in z_min..z_max {
                let idx = i * self.data_width * self.data_height + j * self.data_height + k;
                chunk.set(i, j, k, self.data[idx]);
              }
            }
          }

          results.push(Chunked {
            location: [x_min as f32, y_min as f32, z_min as f32],
            scale: [1., 1., 1.],
            chunk,
          })
        }
      }
    }
    results
  }
}

impl FlatChunker {
  /// Chunk the data into this size
  pub fn with_chunk_size(&self, size: [u16; 3]) -> Self {
    let mut new = self.clone();
    new.chunk_sizes = size;
    new
  }

  /// Chunk data into a fraction of its current size
  // fraction should between 0. and 1.0
  pub fn with_chunk_fraction(&self, fraction: [f32; 3]) -> Self {
    let size: [u16; 3] = [
      (self.data_width as f32 * fraction[0]) as u16,
      (self.data_height as f32 * fraction[1]) as u16,
      (self.data_depth as f32 * fraction[2]) as u16,
    ];
    self.with_chunk_size(size)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::chunk::Chunk;

  #[test]
  fn test_flat_chunker() {
    let dimensions = [320, 320, 320];
    let data = vec![1.; dimensions[0] * dimensions[1] * dimensions[2]];

    let chunker = <FlatChunker as Chunker<Chunk, f32>>::from_array(
      &data,
      dimensions[0],
      dimensions[1],
      dimensions[2],
    )
    .with_chunk_size([16, 16, 16]);

    let results: Vec<Chunked<Chunk>> = chunker.generate_chunks();

    assert_eq!(results.len(), 20);
  }
}
