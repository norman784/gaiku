pub use glam;
use std::fs::File;

mod data;
mod tree;

pub type Color = [u8; 4];
pub type Vector2 = [f32; 2];
pub type Vector3 = [f32; 3];
pub type Vector4 = [f32; 4];

pub use crate::{
  data::{Chunk, Mesh, MeshBuilder},
  tree::{Boundary, Octree},
};

pub trait Baker {
  fn bake(chunk: &Chunk) -> Option<Mesh>;
}

// TODO: Someone points me that is better to use BufReader instead of file or read, need to research about that https://www.reddit.com/r/rust/comments/achili/criticism_and_advices_on_how_to_improve_my_crate/edapxg8
pub trait FileFormat {
  fn load(stream: &mut File) -> Vec<Chunk>;

  fn read(file: &str) -> Vec<Chunk> {
    let mut stream = File::open(file).unwrap();
    Self::load(&mut stream)
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
