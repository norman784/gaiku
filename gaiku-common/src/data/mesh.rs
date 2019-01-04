use super::{
    Vector2,
    Vector3,
    Vector4,
};

#[derive(Debug)]
pub struct Mesh {
    pub indices: Vec<usize>,
    pub vertices: Vec<Vector3>,
    pub normals: Vec<Vector3>,
    pub colors: Vec<Vector4>,
    pub uv: Vec<Vector2>,
    pub tangents: Vec<Vector4>,
}