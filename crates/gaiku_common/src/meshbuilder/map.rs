// This is similar to a grid based method
// except that we use a hashmap so we can
// skip empty nodes
use super::MeshBuilder;
use crate::{boundary::Boundary, mesh::Meshify};
use glam::{Vec2, Vec3};
use std::collections::{hash_map::Values, HashMap};

const EPSILON: f32 = 1e-4;
const COORD_SCALE: f32 = 10.; // 0..1 in input is 0..10 in cache space

/// Helper component that makes easy to build a triangle list mesh.
#[derive(Debug)]
pub struct HashMapBuilder {
  current_index: u32,
  indices: Vec<u32>,
  cache: HashMapTree,
}

impl MeshBuilder for HashMapBuilder {
  /// Crates a new mesh centered at a position and size.
  fn create(_: [f32; 3], _: [f32; 3]) -> Self {
    Self {
      current_index: 0,
      indices: vec![],
      cache: Default::default(),
    }
  }

  /// Inserts the vertice (position, normal, uv and atlas_index) if doesn't exists
  /// and create a new indice for the current data, otherwise retrieves the index of
  /// the input data and inserts the existing index.
  fn add(
    &mut self,
    position: [f32; 3],
    normal: Option<[f32; 3]>,
    uv: Option<[f32; 2]>,
    atlas_index: u16,
  ) {
    let p: Vec3 = position.into();
    let delta: Vec3 = [EPSILON, EPSILON, EPSILON].into();
    let boundary = Boundary::new(&(p - delta), &(p + delta));

    let new = MeshBuilderData::new(
      position.into(),
      normal.map(|d| d.into()),
      uv.map(|d| d.into()),
      atlas_index,
      self.current_index,
    );

    let found_index = {
      let mut found = None;
      for item in self.cache.find_within(boundary) {
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
      self.cache.insert(new);
    }
  }

  fn build<M>(&self) -> Option<M>
  where
    M: Meshify,
  {
    if !self.indices.is_empty() {
      // Load all data into the rstar tree
      // All at once (this is faster then inceremental instertion)
      let mut unsorted_verts: Vec<_> = self
        .cache
        .iter()
        .map(|data| (data.index, data.clone()))
        .collect();
      unsorted_verts.sort_by_key(|data| data.0);
      let verts: Vec<_> = unsorted_verts.into_iter().map(|data| data.1).collect();

      let indices = self.indices.clone();
      let positions: Vec<_> = verts.iter().map(|d| d.position.into()).collect();
      let normals: Vec<_> = verts
        .iter()
        .filter_map(|d| d.normal.map(|d| d.into()))
        .collect();
      let uvs: Vec<_> = verts
        .iter()
        .filter_map(|d| d.uv.map(|d| d.into()))
        .collect();

      Some(M::with(indices, positions, normals, uvs))
    } else {
      None
    }
  }
}

#[derive(Clone, Debug)]
struct MeshBuilderData {
  position: Vec3,
  normal: Option<Vec3>,
  uv: Option<Vec2>,
  atlas_index: u16,
  index: u32,
}

impl PartialEq for MeshBuilderData {
  fn eq(&self, other: &Self) -> bool {
    let pos = (self.position - other.position).length() <= EPSILON;
    if !pos {
      return false;
    }

    let normal = match (self.normal, other.normal) {
      (None, None) => true,
      (None, Some(_)) => false,
      (Some(_), None) => false,
      (Some(a), Some(b)) => (a - b).length() <= EPSILON,
    };
    if !normal {
      return false;
    }

    let uv = match (self.uv, other.uv) {
      (None, None) => true,
      (None, Some(_)) => false,
      (Some(_), None) => false,
      (Some(a), Some(b)) => (a - b).length() <= EPSILON,
    };
    if !uv {
      return false;
    }

    self.atlas_index == other.atlas_index
  }
}
impl Eq for MeshBuilderData {}

impl MeshBuilderData {
  fn new(
    position: Vec3,
    normal: Option<Vec3>,
    uv: Option<Vec2>,
    atlas_index: u16,
    index: u32,
  ) -> Self {
    MeshBuilderData {
      position,
      normal,
      uv,
      atlas_index,
      index,
    }
  }
}

#[derive(Debug, Default)]
struct HashMapNode {
  data: Vec<MeshBuilderData>,
}

impl HashMapNode {
  fn push(&mut self, leaf: MeshBuilderData) {
    self.data.push(leaf);
  }

  fn iter(&self) -> std::slice::Iter<'_, MeshBuilderData> {
    self.data.iter()
  }
}

#[derive(Debug, Default)]
struct HashMapTree {
  data: HashMap<(isize, isize, isize), HashMapNode>,
}

impl HashMapTree {
  fn insert(&mut self, leaf: MeshBuilderData) {
    let p = leaf.position;
    let cache_p = p * COORD_SCALE;
    let cache_idx: [isize; 3] = [
      cache_p[0] as isize,
      cache_p[1] as isize,
      cache_p[2] as isize,
    ];

    self
      .data
      .entry((cache_idx[0], cache_idx[1], cache_idx[2]))
      .or_default()
      .push(leaf);
  }

  fn iter(&self) -> HashMapTreeIter {
    HashMapTreeIter::new(self)
  }

  fn find_within(&self, boundary: Boundary) -> HashMapTreeRangeIter {
    HashMapTreeRangeIter::new(self, boundary)
  }
}

/// This is the iter of a tree.
/// It visits every leaf
struct HashMapTreeIter<'a> {
  inner: Values<'a, (isize, isize, isize), HashMapNode>,
  stack_data: Vec<&'a MeshBuilderData>,
}

impl<'a> HashMapTreeIter<'a> {
  fn new(tree: &'a HashMapTree) -> HashMapTreeIter<'a> {
    HashMapTreeIter {
      inner: tree.data.values(),
      stack_data: vec![],
    }
  }
}

impl<'a> Iterator for HashMapTreeIter<'a> {
  type Item = &'a MeshBuilderData;

  fn next(&mut self) -> Option<Self::Item> {
    while self.stack_data.is_empty() {
      let node = self.inner.next();
      if let Some(node) = node {
        for d in node.data.iter() {
          self.stack_data.push(d);
        }
      } else {
        break;
      }
    }
    self.stack_data.pop()
  }
}

/// This iter visits every leaf in side a boundary
struct HashMapTreeRangeIter<'a> {
  tree: &'a HashMapTree,
  visits: Vec<(isize, isize, isize)>,
  stack_data: Vec<&'a MeshBuilderData>,
  boundary: Boundary,
}

impl<'a> HashMapTreeRangeIter<'a> {
  fn new(tree: &'a HashMapTree, boundary: Boundary) -> HashMapTreeRangeIter<'a> {
    let min = boundary.min;
    let max = boundary.max;
    let cache_min = min * COORD_SCALE;
    let cache_max = max * COORD_SCALE;
    let mut visits = vec![];
    let idx_min = [
      cache_min[0] as isize,
      cache_min[1] as isize,
      cache_min[2] as isize,
    ];
    let idx_max = [
      cache_max[0] as isize + 1,
      cache_max[1] as isize + 1,
      cache_max[2] as isize + 1,
    ];
    for x in idx_min[0]..idx_max[0] {
      for y in idx_min[1]..idx_max[1] {
        for z in idx_min[2]..idx_max[2] {
          visits.push((x, y, z));
        }
      }
    }

    HashMapTreeRangeIter {
      tree,
      visits,
      stack_data: vec![],
      boundary,
    }
  }
}

impl<'a> Iterator for HashMapTreeRangeIter<'a> {
  type Item = &'a MeshBuilderData;

  fn next(&mut self) -> Option<Self::Item> {
    while self.stack_data.is_empty() && !self.visits.is_empty() {
      let next_visit = self.visits.pop();
      if let Some(next_visit) = next_visit {
        if let Some(node) = self.tree.data.get(&next_visit) {
          for data in node.iter() {
            if self.boundary.contains(&data.position) {
              self.stack_data.push(data);
            }
          }
        }
      }
    }
    self.stack_data.pop()
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_hashmaptree_deduplicate() {
    let mut tree = HashMapBuilder::create([0.0, 0.0, 0.0], [4.0, 4.0, 4.0]);

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

    assert_eq!(tree.cache.iter().count(), 4);
  }
}
