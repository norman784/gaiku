// TODO: Add common implementations to read file content and pass to the binary parser implementation
// TODO: Check how amethyst does this
extern crate decorum;

use std::{
    collections::HashMap,
    fs::File,
};

mod data;

pub use self::{
    data::Chunk,
    data::Mesh,
    data::Vec2,
    data::Vec3,
    data::Vec4,
};

pub trait Baker {
    fn bake(chunk: &Chunk) -> Option<Mesh>;

    fn index(vertices: &mut HashMap<Vec3<f32>, usize>, vertex: Vec3<f32>) -> usize {
        let index = vertices.len();
        *vertices.entry(vertex).or_insert(index)
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
