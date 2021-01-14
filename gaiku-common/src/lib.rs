use std::fs::read;

pub use anyhow::Result;
pub use mint;

use crate::{
  boxify::*,
  chunk::Chunkify,
  mesh::Meshify,
  texture::{TextureAtlas2d, Texturify2d},
};

mod boundary;
pub mod boxify;
pub mod chunk;
pub mod mesh;
pub mod texture;
pub mod tree;

pub mod prelude {
  pub use crate::{
    boxify::*,
    chunk::Chunkify,
    mesh::{MeshBuilder, Meshify},
    texture::{TextureAtlas2d, Texturify2d},
    Baker, BakerOptions, FileFormat,
  };
}

pub struct BakerOptions<T>
where
  T: Texturify2d,
{
  pub level_of_detail: usize,
  pub texture: Option<TextureAtlas2d<T>>,
}

impl<T> Default for BakerOptions<T>
where
  T: Texturify2d,
{
  fn default() -> Self {
    Self {
      level_of_detail: 1,
      texture: None,
    }
  }
}

pub trait Baker {
  type Value;

  fn bake<C, T, M>(chunk: &C, options: &BakerOptions<T>) -> Result<Option<M>>
  where
    C: Chunkify<Self::Value> + Sizable,
    T: Texturify2d,
    M: Meshify;
}

// TODO: Someone points me that is better to use BufReader instead of file or read, need to research about that https://www.reddit.com/r/rust/comments/achili/criticism_and_advices_on_how_to_improve_my_crate/edapxg8
pub trait FileFormat {
  type Value;

  fn load<C, T>(bytes: Vec<u8>) -> Result<(Vec<C>, Option<TextureAtlas2d<T>>)>
  where
    C: Chunkify<Self::Value> + Boxify,
    T: Texturify2d;

  fn read<C, T>(file: &str) -> Result<(Vec<C>, Option<TextureAtlas2d<T>>)>
  where
    C: Chunkify<Self::Value> + Boxify,
    T: Texturify2d,
  {
    let bytes = read(file)?;
    Self::load::<C, T>(bytes)
  }
}

/*
pub struct Gaiku {
  terrain: Octree,
}

impl Gaiku {
  pub fn new(data: Vec<Chunk>, size: [f32; 3]) -> Self {
    let mut terrain = Octree::new(size, 8);

    for chunk in data {
      terrain.insert(&chunk);
    }

    Self { terrain }
  }

  pub fn query(&self, boundary: &Boundary) -> Vec<Chunk> {
    self.terrain.query(boundary)
  }

  pub fn get_chunk(&self, point: &Vector3<f32>) -> Option<Chunk> {
    self.terrain.get_leaf(point)
  }

  pub fn set_chunk(&mut self, chunk: &Chunk) -> bool {
    self.terrain.set_leaf(chunk)
  }
}
*/
