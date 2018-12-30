mod chunk;
mod mesh;

pub type Vector2 = [f32; 2];
pub type Vector3 = [f32; 3];
pub type Vector4 = [f32; 4];

pub use self::{
    chunk::Chunk,
    mesh::Mesh,
};