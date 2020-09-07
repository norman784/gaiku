use crate::{Color, Vector2, Vector3};
use glam::Vec3;
use std::collections::HashMap;

#[derive(Default)]
pub struct Mesh {
  pub indices: Vec<u32>,
  pub vertices: Vec<Vector3>,
  pub normals: Vec<Vector3>,
  pub colors: Vec<Color>,
  pub uv: Vec<Vector2>,
}

#[derive(Default)]
pub struct MeshBuilder {
  indices_cache: HashMap<String, u32>,
  indices: Vec<u32>,
  vertices: Vec<Vector3>,
  normals: Vec<Vector3>,
  colors: Vec<Color>,
  uv: Vec<Vector2>,
}

impl MeshBuilder {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn add_triangle_with_color(&mut self, vertices: [Vector3; 3], color: Color) {
    let p0: Vec3 = vertices[0].into();
    let p1: Vec3 = vertices[1].into();
    let p2: Vec3 = vertices[2].into();

    let normal = (p1 - p0).cross(p2 - p0).into();
    for v in vertices.iter() {
      self.add_vertex(v.clone(), Some(normal), Some(color), None);
    }
  }

  fn add_vertex(
    &mut self,
    vertex: Vector3,
    normal: Option<Vector3>,
    color: Option<Color>,
    uv: Option<Vector2>,
  ) -> &mut Self {
    let key = format!("{:?}{:?}{:?}{:?}", vertex, normal, color, uv);
    if let Some(index) = self.indices_cache.get(&key) {
      self.indices.push(*index);
    } else {
      let index = self.indices.len() as u32;
      self.vertices.push(vertex);

      if let Some(normal) = normal {
        self.normals.push(normal);
      }

      if let Some(color) = color {
        self.colors.push(color);
      }

      if let Some(uv) = uv {
        self.uv.push(uv);
      }

      self.indices.push(index);
    }

    self
  }

  pub fn calculate_normals() {
    unimplemented!();
  }

  pub fn build(&self) -> Option<Mesh> {
    if self.indices.len() > 0 {
      Some(Mesh {
        indices: self.indices.clone(),
        vertices: self.vertices.clone(),
        normals: self.normals.clone(),
        colors: self.colors.clone(),
        uv: self.uv.clone(),
      })
    } else {
      None
    }
  }
}
