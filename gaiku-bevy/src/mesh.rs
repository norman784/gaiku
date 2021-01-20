use bevy_render::{
  mesh::{Indices, VertexAttributeValues},
  pipeline::PrimitiveTopology,
  prelude::Mesh,
};
use gaiku_common::prelude::*;

pub struct GaikuMesh {
  indices: Vec<u32>,
  positions: Vec<[f32; 3]>,
  normals: Vec<[f32; 3]>,
  uvs: Vec<[f32; 2]>,
}

impl Meshify for GaikuMesh {
  fn new() -> Self {
    Self::with(vec![], vec![], vec![], vec![])
  }

  fn with(
    indices: Vec<u32>,
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
  ) -> Self {
    Self {
      indices,
      positions,
      normals,
      uvs,
    }
  }

  fn get_indices(&self) -> &Vec<u32> {
    &self.indices
  }

  fn get_normals(&self) -> &Vec<[f32; 3]> {
    &self.normals
  }

  fn get_positions(&self) -> &Vec<[f32; 3]> {
    &self.positions
  }

  fn get_uvs(&self) -> &Vec<[f32; 2]> {
    &self.uvs
  }

  fn set_indices(&mut self, indices: Vec<u32>) {
    self.indices = indices;
  }

  fn set_normals(&mut self, normals: Vec<[f32; 3]>) {
    self.normals = normals;
  }

  fn set_positions(&mut self, positions: Vec<[f32; 3]>) {
    self.positions = positions;
  }

  fn set_uvs(&mut self, uvs: Vec<[f32; 2]>) {
    self.uvs = uvs;
  }
}

impl Into<Mesh> for GaikuMesh {
  fn into(self) -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    if !self.indices.is_empty() {
      mesh.set_indices(Some(Indices::U32(self.indices.clone())));
    }

    mesh.set_attribute(
      Mesh::ATTRIBUTE_POSITION,
      VertexAttributeValues::Float3(self.positions.clone()),
    );

    if !self.normals.is_empty() {
      mesh.set_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::Float3(self.normals.clone()),
      );
    }

    if !self.uvs.is_empty() {
      mesh.set_attribute(
        Mesh::ATTRIBUTE_UV_0,
        VertexAttributeValues::Float2(self.uvs),
      );
    }

    mesh
  }
}
