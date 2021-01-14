mod chunk;
mod sparse_chunk;

pub use chunk::Chunk;
pub use sparse_chunk::SparseChunk;

pub trait Chunkify<T> {
  fn is_air(&self, x: usize, y: usize, z: usize) -> bool;
  fn get(&self, x: usize, y: usize, z: usize) -> T;
  fn set(&mut self, x: usize, y: usize, z: usize, value: T);
}

pub trait ChunkifyMut<T> {
  fn set(&mut self, x: usize, y: usize, z: usize, value: T);
}
