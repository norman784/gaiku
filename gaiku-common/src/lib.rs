// TODO: Add common implementations to read file content and pass to the binary parser implementation
// TODO: Check how amethyst does this
#[macro_use]
extern crate derive_builder;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate getset;
use std::{collections::HashMap, fs::File};
pub use nalgebra;

use nalgebra::Point3;

mod data;
mod tree;

pub use crate::{data::Chunk, data::Mesh};
pub use crate::tree::{Octree, Boundary};

pub trait Baker {
  fn bake(chunk: &Chunk) -> Option<Mesh>;

  // TODO: Creating a string key from the coordinates is not the best solution, enhance this
  fn index(vertices: &mut HashMap<String, (Point3<f32>, usize)>, vertex: Point3<f32>) -> usize {
    let index = vertices.len();
    let key = format!("{}", vertex);
    vertices.entry(key).or_insert((vertex, index)).1
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

pub struct Gaiku {
  terrain: Octree,
}

impl Gaiku {
  pub fn new(data: Vec<Chunk>, size: Point3<f64>) -> Self {
    let mut terrain = Octree::new(size, 8);

    for chunk in  data {
      terrain.insert(&chunk);
    }

    Self {
      terrain
    }
  }

  pub fn query(&self, boundary: &Boundary) -> Vec<Chunk> {
    self.terrain.query(boundary)
  }

  pub fn get_chunk(&self, point: &Point3<f64>) -> Option<Chunk> {
    self.terrain.get_leaf(point)
  }

  pub fn set_chunk(&mut self, chunk: &Chunk) -> bool {
    self.terrain.set_leaf(chunk)
  }
}
