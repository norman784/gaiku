use nalgebra::{Point2, Point3, Point4};

#[derive(Debug, Builder)]
pub struct Mesh {
    pub indices: Vec<usize>,
    pub vertices: Vec<Point3<f32>>,
    #[builder(setter(skip))]
    pub normals: Vec<Point3<f32>>,
    #[builder(setter(skip))]
    pub colors: Vec<Point4<f32>>,
    #[builder(setter(skip))]
    pub uv: Vec<Point2<f32>>,
    #[builder(setter(skip))]
    pub tangents: Vec<Point4<f32>>,
}
