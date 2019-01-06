// TODO: Add common implementations to read file content and pass to the binary parser implementation
// TODO: Check how amethyst does this

use std::fs::File;

mod data;

pub use self::{
    data::Chunk,
    data::Mesh,
    data::Vec2,
    data::Vec3,
    data::Vec4,
};

pub trait Baker {
    fn bake(chunks: &Chunk) -> Option<Mesh>;
}

pub trait Fileformat {
    fn load(stream: &mut File) -> Vec<Chunk>;

    fn read(file: &str) -> Vec<Chunk> {
        let mut stream = File::open(file).unwrap();
        Self::load(&mut stream)
    }
}