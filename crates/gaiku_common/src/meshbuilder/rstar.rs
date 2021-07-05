use super::meshbuilder::MeshBuilder;
use crate::mesh::Meshify;
use glam::Vec3;
use rstar::{RTree, RTreeObject, AABB};
use std::convert::TryInto;

pub struct RstarMeshBuilder {
  tree: RTree<MeshBuilderData, rstar::DefaultParams>,
  indices: Vec<u32>,
}

const EPSILON: f32 = 1e-4;

#[derive(Clone, Debug)]
struct MeshBuilderData {
  position: [f32; 3],
  normal: Option<[f32; 3]>,
  uv: Option<[f32; 2]>,
  atlas_index: u16,
  index: u32,
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

impl RTreeObject for MeshBuilderData {
  type Envelope = AABB<[f32; 3]>;

  fn envelope(&self) -> Self::Envelope {
    let corner_1 = [
      self.position[0] - EPSILON,
      self.position[1] - EPSILON,
      self.position[2] - EPSILON,
    ];
    let corner_2 = [
      self.position[0] + EPSILON,
      self.position[1] + EPSILON,
      self.position[2] + EPSILON,
    ];
    AABB::from_corners(corner_1, corner_2)
  }
}

// If we ever want more precise distant point data from
// the rstar tree we can use this.
// impl PointDistance for MeshBuilderData {
//   fn distance_2(&self, point: &[f32; 3]) -> f32 {
//     let d_x = self.position[0] - point[0];
//     let d_y = self.position[1] - point[1];
//     let d_z = self.position[2] - point[2];
//     let distance_to_origin = (d_x * d_x + d_y * d_y + d_z * d_z).sqrt();
//     // So that contains_point works we must return 0.
//     // if it is within the EPSILON distance
//     let distance_to_ring = distance_to_origin - EPSILON;
//     let distance_to_circle = f32::max(0.0, distance_to_ring);
//     // We must return the squared distance!
//     distance_to_circle * distance_to_circle
//   }
//
//   // This implementation is not required but more efficient since it
//   // omits the calculation of a square root
//   fn contains_point(&self, point: &[f32; 3]) -> bool {
//     let d_x = self.position[0] - point[0];
//     let d_y = self.position[1] - point[1];
//     let d_z = self.position[2] - point[2];
//     let distance_to_origin_2 = d_x * d_x + d_y * d_y + d_z * d_z;
//     let radius_2 = EPSILON; // No point squaring this as 1e-4*1e-4 is 1e-8 which is too small
//     distance_to_origin_2 <= radius_2
//   }
// }

impl MeshBuilder for RstarMeshBuilder {
  /// Crates a new mesh centered at a position and size.
  fn create(_: [f32; 3], _: [f32; 3]) -> Self {
    Self {
      tree: RTree::new(),
      indices: vec![],
    }
  }

  /// Inserts the vertice (position, normal, uv and atlas_index)
  fn add(
    &mut self,
    position: [f32; 3],
    normal: Option<[f32; 3]>,
    uv: Option<[f32; 2]>,
    atlas_index: u16,
  ) {
    let index: u32 = self.tree.size().try_into().unwrap();
    let new = MeshBuilderData {
      position,
      normal,
      uv,
      atlas_index,
      index,
    };
    let found_index = {
      let mut found = None;
      let p: Vec3 = position.into();
      let delta: Vec3 = [EPSILON, EPSILON, EPSILON].into();
      let envelope = AABB::from_corners((p - delta).into(), (p + delta).into());
      for item in self.tree.locate_in_envelope_intersecting(&envelope) {
        if item == &new {
          found = Some(item.index);
          break;
        }
      }
      found
    };
    if let Some(found_index) = found_index {
      self.indices.push(found_index);
    } else {
      self.indices.push(index);
      self.tree.insert(new);
    }
  }

  fn build<M>(&self) -> Option<M>
  where
    M: Meshify,
  {
    if self.indices.len() > 0 {
      // Load all data into the rstar tree
      // All at once (this is faster then inceremental instertion)
      let mut unsorted_verts: Vec<_> = self
        .tree
        .iter()
        .map(|data| (data.index, data.clone()))
        .collect();
      unsorted_verts.sort_by_key(|data| data.0);
      let verts: Vec<_> = unsorted_verts.into_iter().map(|data| data.1).collect();

      let indices = self.indices.clone();
      let positions: Vec<_> = verts.iter().map(|d| d.position.clone()).collect();
      let normals: Vec<_> = verts.iter().filter_map(|d| d.normal.clone()).collect();
      let uvs: Vec<_> = verts.iter().filter_map(|d| d.uv.clone()).collect();

      Some(M::with(indices, positions, normals, uvs))
    } else {
      None
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_rstar_deduplicate() {
    let mut tree = RstarMeshBuilder::create([0.0, 0.0, 0.0], [4.0, 4.0, 4.0]);

    for _ in 0..10 {
      tree.add([0., 0., 0.], None, None, 0);
    }

    for _ in 0..10 {
      tree.add([1., 0., 0.], None, None, 0);
    }

    for _ in 0..10 {
      tree.add([1., 1., 0.], None, None, 0);
    }

    for _ in 0..10 {
      tree.add([0., 1., 0.], None, None, 0);
    }

    assert_eq!(tree.tree.size(), 4);
  }
}
