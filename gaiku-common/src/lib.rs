// TODO: Add common implementations to read file content and pass to the binary parser implementation
// TODO: Check how amethyst does this
use std::{collections::HashMap, fs::File};

pub use acacia;
pub use nalgebra;

use nalgebra::Point3;
//use acacia::Tree;

mod data;

pub use self::{data::Chunk, data::Mesh};

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
//    tree: Tree,
}
