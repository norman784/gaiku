mod chunk;
mod mesh;

#[derive(Debug, Copy, Clone)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl From<[f32; 2]> for Vector2 {
    fn from(value: [f32; 2]) -> Self {
        Vector2 { x:  value[0], y: value[1] }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<[f32; 3]> for Vector3 {
    fn from(value: [f32; 3]) -> Self {
        Vector3 { x:  value[0], y: value[1], z: value[2] }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector3i {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl From<[i32; 3]> for Vector3i {
    fn from(value: [i32; 3]) -> Self {
        Vector3i { x:  value[0], y: value[1], z: value[2] }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl From<[f32; 4]> for Vector4 {
    fn from(value: [f32; 4]) -> Self {
        Vector4 { x:  value[0], y: value[1], z: value[2], w: value[3] }
    }
}

pub use self::{
    chunk::Chunk,
    mesh::Mesh,
};