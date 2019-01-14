use super::{Vec2, Vec3, Vec4};

#[derive(Debug)]
pub struct Mesh {
    pub indices: Vec<usize>,
    pub vertices: Vec<Vec3<f32>>,
    pub normals: Vec<Vec3<f32>>,
    pub colors: Vec<Vec4<f32>>,
    pub uv: Vec<Vec2<f32>>,
    pub tangents: Vec<Vec4<f32>>,
}
