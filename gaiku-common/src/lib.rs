use std::fs::read;

pub use anyhow::Result;
pub use mint;

use mint::Vector3;

mod data;
mod tree;

pub use crate::{
  data::{mesh, texture, Chunk, Chunkify, Mesh, MeshBuilder, Texture2d, TextureAtlas2d},
  tree::{Boundary, Octree},
};

pub mod prelude {
  pub use crate::{
    data::{Chunkify, Mesh, MeshBuilder},
    Baker, BakerOptions, FileFormat,
  };
}

pub struct BakerOptions {
  pub level_of_detail: usize,
  pub texture: Option<TextureAtlas2d>,
}

impl Default for BakerOptions {
  fn default() -> Self {
    Self {
      level_of_detail: 1,
      texture: None,
    }
  }
}

pub trait Baker {
  fn bake(chunk: &Chunk, options: &BakerOptions) -> Result<Option<Mesh>>;
}

// TODO: Someone points me that is better to use BufReader instead of file or read, need to research about that https://www.reddit.com/r/rust/comments/achili/criticism_and_advices_on_how_to_improve_my_crate/edapxg8
pub trait FileFormat {
  fn load(bytes: Vec<u8>) -> Result<(Vec<Chunk>, Option<TextureAtlas2d>)>;

  fn read(file: &str) -> Result<(Vec<Chunk>, Option<TextureAtlas2d>)> {
    let bytes = read(file)?;
    Self::load(bytes)
  }
}

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
