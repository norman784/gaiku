use nalgebra::{Point2, Point3, Point4};

#[derive(Debug)]
pub struct Mesh {
    pub indices: Vec<usize>,
    pub vertices: Vec<Point3<f32>>,
    pub normals: Vec<Point3<f32>>,
    pub colors: Vec<Point4<f32>>,
    pub uv: Vec<Point2<f32>>,
    pub tangents: Vec<Point4<f32>>,
}
