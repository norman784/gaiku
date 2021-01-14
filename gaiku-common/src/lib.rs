pub use glam;
pub use mint;
use std::{collections::HashMap, fs::File};

use mint::Vector3;

mod data;
mod tree;

pub use crate::{
  data::{Chunk, Chunkify, ChunkifyMut, ChunkifyNeighboured, Mesh},
  tree::{Boundary, Octree},
};

pub trait Baker {
  fn bake<T: Chunkify>(chunk: &T) -> Option<Mesh>;

  // TODO: Creating a string key from the coordinates is not the best solution, enhance this
  fn index(vertices: &mut HashMap<String, (Vector3<f32>, u16)>, vertex: Vector3<f32>) -> u16 {
    let index = vertices.len();
    let key = format!("{:?}", vertex);
    vertices.entry(key).or_insert((vertex, index as u16)).1
  }
}

// TODO: Someone points me that is better to use BufReader instead of file or read, need to research about that https://www.reddit.com/r/rust/comments/achili/criticism_and_advices_on_how_to_improve_my_crate/edapxg8
pub trait FileFormat {
  fn load(stream: &mut File) -> Vec<Chunk>;

  fn read(file: &str) -> Vec<Chunk> {
    let mut stream = File::open(file).unwrap();
    Self::load(&mut stream)
  }
}

pub struct Gaiku<T> {
  terrain: Octree<T>,
}

impl<T> Gaiku<T>
where
  T: Chunkify + ChunkifyNeighboured + Clone,
{
  pub fn new(data: Vec<T>, size: Vector3<f32>) -> Self {
    let mut terrain = Octree::new(size, 8);

    for chunk in data {
      terrain.insert(&chunk);
    }

    Self { terrain }
  }

  pub fn query(&self, boundary: &Boundary) -> Vec<T> {
    self.terrain.query(boundary)
  }

  pub fn get_chunk(&self, point: &Vector3<f32>) -> Option<T> {
    self.terrain.get_leaf(point)
  }

  pub fn set_chunk(&mut self, chunk: &T) -> bool {
    self.terrain.set_leaf(chunk)
  }
}
