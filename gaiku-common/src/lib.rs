// TODO: Add common implementations to read file content and pass to the binary parser implementation
// TODO: Check how amethyst does this
#[macro_use]
extern crate typed_builder;
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate getset;
pub use glam;
pub use mint;
use std::{collections::HashMap, fs::File};

use mint::Vector3;

mod data;
mod tree;

pub use crate::tree::{Boundary, Octree};
pub use crate::{data::Chunk, data::Mesh};

pub trait Baker {
    fn bake(chunk: &Chunk) -> Option<Mesh>;

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

pub struct Gaiku {
    terrain: Octree,
}

impl Gaiku {
    pub fn new(data: Vec<Chunk>, size: Vector3<f64>) -> Self {
        let mut terrain = Octree::new(size, 8);

        for chunk in data {
            terrain.insert(&chunk);
        }

        Self { terrain }
    }

    pub fn query(&self, boundary: &Boundary) -> Vec<Chunk> {
        self.terrain.query(boundary)
    }

    pub fn get_chunk(&self, point: &Vector3<f64>) -> Option<Chunk> {
        self.terrain.get_leaf(point)
    }

    pub fn set_chunk(&mut self, chunk: &Chunk) -> bool {
        self.terrain.set_leaf(chunk)
    }
}
