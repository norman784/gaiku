// Takes input data and generates a
// series of chunks at specified size
use super::common::*;
use crate::{
  atlas::AtlasifyMut,
  boxify::Boxify,
  chunk::{Chunkify, ChunkifyMut},
};
use std::convert::TryInto;

///
/// FlatChunker will simply seperate the data into chunks of a
/// specified size.
///
/// If the size of the data does not equally
/// divide into the specified size then the last chunks
/// in each dimension will be smaller.
///
pub struct FlatChunker<C> {
  data: Vec<f32>,
  atlas_data: Vec<u8>,
  data_width: usize,
  data_height: usize,
  data_depth: usize,
  chunk_sizes: [u16; 3],
  results: Vec<Chunked<C>>,
}

impl<C> Chunker<C, f32, u8> for FlatChunker<C>
where
  C: Chunkify<f32> + ChunkifyMut<f32> + AtlasifyMut<u8> + Boxify,
{
  fn from_array_with_atlas(
    data: &[f32],
    atlas_data: &[u8],
    width: usize,
    height: usize,
    depth: usize,
  ) -> Self {
    Self {
      data: data.to_vec(),
      atlas_data: atlas_data.to_vec(),
      data_width: width,
      data_height: height,
      data_depth: depth,
      chunk_sizes: [16, 16, 16],
      results: vec![],
    }
  }

  ///
  /// Generates the chunks from the source data
  ///
  /// The source data needs to be already setup
  /// and ready before making this call
  ///
  /// # Examples
  ///
  /// ```
  /// use crate::prelude::*;
  ///
  /// let dimensions = [48, 48, 48];
  /// let data = vec![1.; dimensions[0] * dimensions[1] * dimensions[2]];
  ///
  /// let mut chunker: FlatChunker<Chunk> = FlatChunker::from_array(
  ///   &data,
  ///   dimensions[0],
  ///   dimensions[1],
  ///   dimensions[2],
  /// );
  ///
  /// chunker.generate_chunks();
  /// let results = chunker.get_chunks();
  ///
  /// assert_eq!(results.len(), 27);
  /// ```
  ///
  fn generate_chunks(&mut self) {
    let mut results = vec![];
    let chunk_sizes = [
      self.chunk_sizes[0] as usize,
      self.chunk_sizes[1] as usize,
      self.chunk_sizes[2] as usize,
    ];
    for x in 0..(self.data_width / chunk_sizes[0] + 1) {
      for y in 0..(self.data_height / chunk_sizes[1] + 1) {
        for z in 0..(self.data_depth / chunk_sizes[2] + 1) {
          let x_min = x * chunk_sizes[0];
          let x_max = std::cmp::min(x_min + chunk_sizes[0] + 1, self.data_width);
          let x_size = x_max - x_min;
          if x_size == 0 {
            continue;
          }

          let y_min = y * chunk_sizes[1];
          let y_max = std::cmp::min(y_min + chunk_sizes[1] + 1, self.data_height);
          let y_size = y_max - y_min;
          if y_size == 0 {
            continue;
          }

          let z_min = z * chunk_sizes[2];
          let z_max = std::cmp::min(z_min + chunk_sizes[2] + 1, self.data_depth);
          let z_size = z_max - z_min;
          if z_size == 0 {
            continue;
          }
          let location = [x_min as f32, y_min as f32, z_min as f32];
          let mut chunk = C::new(
            location,
            x_size.try_into().unwrap(),
            y_size.try_into().unwrap(),
            z_size.try_into().unwrap(),
          );

          for i in x_min..x_max {
            let c_i = i - x_min;
            for j in y_min..y_max {
              let c_j = j - y_min;
              for k in z_min..z_max {
                let c_k = k - z_min;
                let idx = i + j * self.data_width + k * self.data_width * self.data_height;
                chunk.set(c_i, c_j, c_k, self.data[idx]);
                if let Some(&atlas) = self.atlas_data.get(idx) {
                  chunk.set_atlas(c_i, c_j, c_k, atlas);
                }
              }
            }
          }

          results.push(Chunked {
            location,
            scale: [1., 1., 1.],
            chunk,
          })
        }
      }
    }
    self.results = results;
  }

  ///
  /// Gets the resulting chunks
  ///
  /// This should be called after `generate_chunks`
  ///
  /// # Returns
  ///
  /// returns a `Vec<& Chunked<C>>`
  ///
  fn get_chunks(&self) -> Vec<&Chunked<C>> {
    self.results.iter().collect()
  }

  ///
  /// Gets the resulting chunks mutably
  ///
  /// This should be called after `generate_chunks`
  ///
  /// # Returns
  ///
  /// returns a `Vec<&mut Chunked<C>>`
  ///
  fn get_chunks_mut(&mut self) -> Vec<&mut Chunked<C>> {
    self.results.iter_mut().collect()
  }
}

impl<C> FlatChunker<C>
where
  C: Chunkify<f32> + ChunkifyMut<f32> + AtlasifyMut<u8> + Boxify,
{
  ///
  /// Sets the size to use for this chunker
  /// chuked size
  ///
  /// long description
  ///
  /// # Parameters
  ///
  /// * `size` - maximum size of generated chunks in `[u16; 3]`
  ///
  /// # Returns
  ///
  /// returns Self
  ///
  /// # Examples
  ///
  /// ```
  /// use crate::prelude::*;
  ///
  /// let dimensions = [48, 48, 48];
  /// let data = vec![1.; dimensions[0] * dimensions[1] * dimensions[2]];
  ///
  /// let mut chunker: FlatChunker<Chunk> = FlatChunker::from_array(
  ///   &data,
  ///   dimensions[0],
  ///   dimensions[1],
  ///   dimensions[2],
  /// );
  /// chunker.set_chunk_size([16, 16, 16]);
  ///
  /// chunker.generate_chunks();
  /// let results = chunker.get_chunks();
  /// assert_eq!(results.len(), 27);
  /// ```
  ///
  pub fn set_chunk_size(&mut self, size: [u16; 3]) {
    self.chunk_sizes = size;
  }

  /// Chunk data into a fraction of its current size
  ///
  /// # Parameters
  ///
  /// * `fraction` - Should be a float between 0. and 1.
  ///
  /// # Returns
  ///
  /// returns Self
  ///
  /// # Examples
  ///
  /// ```
  /// use crate::prelude::*;
  ///
  /// let dimensions = [48, 48, 48];
  /// let data = vec![1.; dimensions[0] * dimensions[1] * dimensions[2]];
  ///
  /// let mut chunker: FlatChunker<Chunk> = FlatChunker::from_array(
  ///   &data,
  ///   dimensions[0],
  ///   dimensions[1],
  ///   dimensions[2],
  /// );
  /// chunker.set_chunk_fraction(0.5);
  ///
  /// chunker.generate_chunks();
  /// let results = chunker.get_chunks();
  /// assert_eq!(results.len(), 8);
  /// ```
  ///
  pub fn set_chunk_fraction(&mut self, fraction: [f32; 3]) {
    let size: [u16; 3] = [
      (self.data_width as f32 * fraction[0]) as u16,
      (self.data_height as f32 * fraction[1]) as u16,
      (self.data_depth as f32 * fraction[2]) as u16,
    ];
    self.set_chunk_size(size);
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::chunk::Chunk;

  #[test]
  fn test_flat_chunker() {
    let dimensions = [48, 48, 48];
    let data = vec![1.; dimensions[0] * dimensions[1] * dimensions[2]];

    let mut chunker: FlatChunker<Chunk> =
      FlatChunker::from_array(&data, dimensions[0], dimensions[1], dimensions[2]);
    chunker.set_chunk_size([16, 16, 16]);

    chunker.generate_chunks();
    let results = chunker.get_chunks();

    assert_eq!(results.len(), 27);
  }
}
