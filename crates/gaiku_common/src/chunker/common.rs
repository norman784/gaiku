use crate::{
  boxify::Sizable,
  chunk::{Chunkify, ChunkifyMut},
};
use std::convert::TryInto;

pub struct Chunked<C> {
  pub location: [f32; 3],
  pub scale: [f32; 3],
  pub chunk: C,
}

pub trait Chunker<C, T>
where
  C: Chunkify<T> + ChunkifyMut<T> + Sizable,
{
  fn from_chunk<D>(chunk: &D) -> Self
  where
    D: Chunkify<T> + Sizable,
    Self: Sized,
  {
    let width = chunk.width();
    let height = chunk.height();
    let depth = chunk.depth();
    let mut data = vec![];
    for x in 0..width {
      for y in 0..height {
        for z in 0..depth {
          data.push(chunk.get(x.into(), y.into(), z.into()));
        }
      }
    }

    Self::from_array(
      data.as_slice(),
      width.try_into().unwrap(),
      height.try_into().unwrap(),
      depth.try_into().unwrap(),
    )
  }

  fn from_array(data: &[T], width: usize, height: usize, depth: usize) -> Self
  where
    Self: Sized;

  fn generate_chunks(&self) -> Vec<Chunked<C>>;
}
