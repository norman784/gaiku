use mint::{Vector2, Vector3, Vector4};

#[derive(Debug, Builder)]
pub struct Mesh {
    pub indices: Vec<usize>,
    pub vertices: Vec<Vector3<f32>>,
    #[builder(setter(skip))]
    pub normals: Vec<Vector3<f32>>,
    #[builder(setter(skip))]
    pub colors: Vec<Vector4<f32>>,
    #[builder(setter(skip))]
    pub uv: Vec<Vector2<f32>>,
    #[builder(setter(skip))]
    pub tangents: Vec<Vector4<f32>>,
}
