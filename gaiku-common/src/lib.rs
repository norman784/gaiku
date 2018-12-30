// TODO: Add common implementations to read file content and pass to the binary parser implementation
// TODO: Check how amethyst does this

use std::fs::File;

mod data;

pub use self::{
    data::Chunk,
    data::Mesh,
    data::Vector3,
};

pub trait Baker {
    fn bake(chunks: &Chunk) -> Mesh;
}

pub trait Fileformat {
    fn load(stream: &File) -> Vec<Chunk>;

    fn read(file: &str) -> Vec<Chunk> {
        let stream = &(File::open(file).unwrap());
        Self::load(&stream)
    }
}