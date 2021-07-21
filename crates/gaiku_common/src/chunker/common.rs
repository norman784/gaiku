use crate::{
  atlas::{Atlasify, AtlasifyMut},
  boxify::{Boxify, Sizable},
  chunk::{Chunkify, ChunkifyMut},
};
use std::convert::TryInto;

///
/// Chunk holds the result from a chunking operation
///
/// This includes aspects like location and scale
/// because some chunking operations such as the LOD
/// chunkers will scale the chunks to adjust the resolution
///
pub struct Chunked<C> {
  /// The location of the chunk relative to the first sample datas location
  pub location: [f32; 3],
  /// The scale of the chunk relative to the original datas scale
  pub scale: [f32; 3],
  /// The resulting chunk
  pub chunk: C,
}

///
/// Generic interface for Chunkers
///
pub trait Chunker<C, T, U>
where
  C: Chunkify<T> + ChunkifyMut<T> + AtlasifyMut<U> + Boxify,
{
  ///
  /// Create chunks using this chunk for input
  ///
  /// The values and atlas values from the source
  /// chunk are copied
  ///
  /// # Parameters
  ///
  /// * `chunk` - chunk
  ///
  /// # Returns
  ///
  /// returns description
  ///
  fn from_chunk<D>(chunk: &D) -> Self
  where
    D: Chunkify<T> + Atlasify<U> + Sizable,
    Self: Sized,
  {
    let width = chunk.width();
    let height = chunk.height();
    let depth = chunk.depth();
    let mut data = vec![];
    let mut atlas_data = vec![];
    for x in 0..width {
      for y in 0..height {
        for z in 0..depth {
          data.push(chunk.get(x.into(), y.into(), z.into()));
          atlas_data.push(chunk.get_atlas(x.into(), y.into(), z.into()));
        }
      }
    }

    Self::from_array_with_atlas(
      data.as_slice(),
      atlas_data.as_slice(),
      width.try_into().unwrap(),
      height.try_into().unwrap(),
      depth.try_into().unwrap(),
    )
  }

  ///
  /// Chunks the input array
  ///
  /// The length of data samples should match those provided by
  /// width * height * depth
  ///
  /// # Parameters
  ///
  /// * `data` - The input data samples
  ///
  /// * `width` - width of samples
  ///
  /// * `height` - height of samples
  ///
  /// * `depth` - depth of samples
  ///
  /// # Returns
  ///
  /// returns description
  ///
  fn from_array(data: &[T], width: usize, height: usize, depth: usize) -> Self
  where
    Self: Sized,
  {
    Self::from_array_with_atlas(data, &[], width, height, depth)
  }

  ///
  /// Chunks the data with atlas values
  ///
  /// The data needs to have the length equal to the
  /// width * height * depth. But the atlas data does
  /// not any missing values will likely be replaced with
  /// defaults
  ///
  /// # Parameters
  ///
  /// * `data` - The source data for the values
  ///
  /// * `atlas_data` - The source data for the atlas values
  ///
  /// * `width` - width of the data
  ///
  /// * `height` - height of the data
  ///
  /// * `depth` - depth of the data
  ///
  /// # Returns
  ///
  /// returns description
  ///
  fn from_array_with_atlas(
    data: &[T],
    atlas_data: &[U],
    width: usize,
    height: usize,
    depth: usize,
  ) -> Self
  where
    Self: Sized;

  ///
  /// Generates the chunks
  ///
  /// The source data needs to be already setup
  /// and ready before making this call
  ///
  fn generate_chunks(&mut self);

  ///
  /// Gets the resulting chunks
  ///
  /// This should be called after `generate_chunks`
  ///
  /// # Returns
  ///
  /// returns a `Vec<& Chunked<C>>`
  ///
  fn get_chunks(&self) -> Vec<&Chunked<C>>;

  ///
  /// Gets the resulting chunks mutably
  ///
  /// This should be called after `generate_chunks`
  ///
  /// # Returns
  ///
  /// returns a `Vec<&mut Chunked<C>>`
  ///
  fn get_chunks_mut(&mut self) -> Vec<&mut Chunked<C>>;
}
