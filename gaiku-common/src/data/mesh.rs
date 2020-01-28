use mint::{Vector2, Vector3, Vector4};

#[derive(Clone, Debug, TypedBuilder)]
pub struct Mesh {
    pub indices: Vec<u16>,
    pub vertices: Vec<Vector3<f32>>,
    #[builder(default)]
    pub normals: Vec<Vector3<f32>>,
    #[builder(default)]
    pub colors: Vec<Vector4<f32>>,
    #[builder(default)]
    pub uv: Vec<Vector2<f32>>,
    #[builder(default)]
    pub tangents: Vec<Vector4<f32>>,
}
