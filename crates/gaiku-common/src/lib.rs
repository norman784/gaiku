pub use crate::{
  data::{Chunk, Mesh, MeshBuilder},
  tree::{Boundary, Octree},
};
use anyhow::Result;
use std::fs::read;

mod data;
mod tree;

pub type Color = [u8; 4];
pub type Vector2 = [f32; 2];
pub type Vector3 = [f32; 3];
pub type Vector4 = [f32; 4];

pub trait Baker {
  fn bake(chunk: &Chunk) -> Option<Mesh>;
}

// TODO: Someone points me that is better to use BufReader instead of file or read, need to research about that https://www.reddit.com/r/rust/comments/achili/criticism_and_advices_on_how_to_improve_my_crate/edapxg8
pub trait FileFormat {
  fn from_bytes(bytes: Vec<u8>) -> Result<Vec<Chunk>>;

  fn load_file(file: &str) -> Result<Vec<Chunk>> {
    Self::from_bytes(read(file)?)
  }
}

pub struct Gaiku {
  terrain: Octree,
}

impl Gaiku {
  pub fn new(data: Vec<Chunk>, size: Vector3) -> Self {
    let mut terrain = Octree::new(size, 8);

    for chunk in data {
      terrain.insert(&chunk);
    }

    Self { terrain }
  }

  pub fn from_bytes<T: FileFormat>(bytes: Vec<u8>, size: Vector3) -> Result<Self> {
    let chunks = T::from_bytes(bytes)?;
    Ok(Self::new(chunks, size))
  }

  pub fn load<T: FileFormat>(path: &str, size: Vector3) -> Result<Self> {
    let chunks = T::load_file(path)?;
    Ok(Self::new(chunks, size))
  }

  pub fn bake<T: Baker>(&self, area: &Boundary) -> Vec<(Mesh, Vector3)> {
    self
      .query(area)
      .iter()
      .map(|c| (T::bake(c), c.position()))
      .filter(|(m, _)| m.is_some())
      .map(|(m, p)| (m.unwrap(), p))
      .collect::<Vec<(Mesh, Vector3)>>()
  }

  pub fn bake_all<T: Baker>(&self) -> Vec<(Mesh, Vector3)> {
    let boundary = Boundary::new([0., 0., 0.], self.terrain.size());
    self.bake::<T>(&boundary)
  }

  pub fn query(&self, boundary: &Boundary) -> Vec<Chunk> {
    self.terrain.query(boundary)
  }

  pub fn get_chunk(&self, point: &Vector3) -> Option<Chunk> {
    self.terrain.get_leaf(point)
  }

  pub fn set_chunk(&mut self, chunk: &Chunk) -> bool {
    self.terrain.set_leaf(chunk)
  }
}
