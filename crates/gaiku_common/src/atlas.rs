#[allow(clippy::module_inception)]
/// Base common denominator across all the chunk implementations used.
pub trait Atlasify<T> {
  fn get_atlas(&self, x: usize, y: usize, z: usize) -> T;
}

/// Defines a mutable chunk.
pub trait AtlasifyMut<T> {
  fn set_atlas(&mut self, x: usize, y: usize, z: usize, value: T);
}
