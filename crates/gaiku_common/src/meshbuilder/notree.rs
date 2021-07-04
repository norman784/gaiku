use super::meshbuilder::MeshBuilder;
use crate::mesh::Meshify;
use std::convert::TryInto;

pub struct NoTreeBuilder {
  data: Vec<MeshBuilderData>,
}

const EPSILON: f32 = 1e-4;

#[derive(Clone, Debug)]
struct MeshBuilderData {
  position: [f32; 3],
  normal: Option<[f32; 3]>,
  uv: Option<[f32; 2]>,
  atlas_index: u16,
}

impl PartialEq for MeshBuilderData {
  fn eq(&self, other: &Self) -> bool {
    let pos = (self.position[0] - other.position[0]).abs() <= EPSILON
      && (self.position[1] - other.position[1]).abs() <= EPSILON
      && (self.position[2] - other.position[2]).abs() <= EPSILON;
    if pos == false {
      return false;
    }

    let normal = match (self.normal, other.normal) {
      (None, None) => true,
      (None, Some(_)) => false,
      (Some(_), None) => false,
      (Some(a), Some(b)) => {
        (a[0] - b[0]).abs() <= EPSILON
          && (a[1] - b[1]).abs() <= EPSILON
          && (a[2] - b[2]).abs() <= EPSILON
      }
    };
    if normal == false {
      return false;
    }

    let uv = match (self.uv, other.uv) {
      (None, None) => true,
      (None, Some(_)) => false,
      (Some(_), None) => false,
      (Some(a), Some(b)) => (a[0] - b[0]).abs() <= EPSILON && (a[1] - b[1]).abs() <= EPSILON,
    };
    if uv == false {
      return false;
    }

    self.atlas_index == other.atlas_index
  }
}
impl Eq for MeshBuilderData {}

impl MeshBuilder for NoTreeBuilder {
  /// Crates a new mesh centered at a position and size.
  fn create(_: [f32; 3], _: [f32; 3]) -> Self {
    Self { data: vec![] }
  }

  /// Inserts the vertice (position, normal, uv and atlas_index)
  fn add(
    &mut self,
    position: [f32; 3],
    normal: Option<[f32; 3]>,
    uv: Option<[f32; 2]>,
    atlas_index: u16,
  ) {
    let new = MeshBuilderData {
      position,
      normal,
      uv,
      atlas_index,
    };
    self.data.push(new);
  }

  fn build<M>(&self) -> Option<M>
  where
    M: Meshify,
  {
    if self.data.len() > 0 {
      // Load all data into the rstar tree
      // All at once (this is faster then inceremental instertion)

      let indices: Vec<u32> = (0..self.data.len())
        .into_iter()
        .map(|i| i.try_into().unwrap())
        .collect();
      let positions: Vec<_> = self.data.iter().map(|d| d.position.clone()).collect();
      let normals: Vec<_> = self.data.iter().filter_map(|d| d.normal.clone()).collect();
      let uvs: Vec<_> = self.data.iter().filter_map(|d| d.uv.clone()).collect();

      Some(M::with(indices, positions, normals, uvs))
    } else {
      None
    }
  }
}
