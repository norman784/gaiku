mod map;
mod notree;
mod octree;
mod rstar;

pub use self::{
  map::HashMapBuilder, notree::NoTreeBuilder, octree::OctMeshBuilder, rstar::RstarMeshBuilder,
};

pub type DefaultMeshBuilder = OctMeshBuilder;

use crate::Meshify;

pub trait MeshBuilder {
  /// Crates a new mesh centered at a position and size.
  fn create(center: [f32; 3], size: [f32; 3]) -> Self;

  /// Inserts the vertice (position, normal, uv and atlas_index)
  fn add(
    &mut self,
    position: [f32; 3],
    normal: Option<[f32; 3]>,
    uv: Option<[f32; 2]>,
    atlas_index: u16,
  );

  /// Inserts the triangle and generate the index if needed, otherwise use an existing index.
  /// The triangle data is expected to be counter-clockwise.
  fn add_triangle(
    &mut self,
    triangle: [[f32; 3]; 3],
    normal: Option<[f32; 3]>,
    uv: Option<[[f32; 2]; 3]>,
    atlas_index: u16,
  ) {
    for (i, vertex) in triangle.iter().enumerate() {
      self.add(*vertex, normal, uv.map(|uv| uv[i]), atlas_index);
    }
  }

  /// Inserts the face (generates 2  triangles) and generate the index if needed,
  /// otherwise use an existing index. The face data is expected to be counter-clockwise.
  fn add_face(
    &mut self,

    face: [[f32; 3]; 4],
    normal: Option<[f32; 3]>,
    uv: Option<[[f32; 2]; 4]>,
    atlas_index: u16,
  ) {
    [[0, 1, 3], [1, 2, 3]].iter().for_each(|triangle| {
      triangle.iter().for_each(|i| {
        self.add(face[*i], normal, uv.map(|uv| uv[*i]), atlas_index);
      });
    });
  }

  fn build<M>(&self) -> Option<M>
  where
    M: Meshify;
}
