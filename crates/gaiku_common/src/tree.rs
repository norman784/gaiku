use crate::{boundary::Boundary, boxify::*, chunk::Chunk};
use mint::Vector3;

pub type Octree = Tree;

#[derive(Clone, Debug)]
pub struct Node {
  boundary: Boundary,
  bucket: usize,
  leafs: Option<Vec<Chunk>>,
  nodes: Option<Vec<Node>>,
}

impl Node {
  fn new(boundary: Boundary, bucket: usize) -> Self {
    Node {
      boundary,
      bucket,
      leafs: None,
      nodes: None,
    }
  }

  fn insert(&mut self, leaf: &Chunk) -> bool {
    if !self.boundary.contains(&leaf.position().into()) {
      return false;
    }

    match &mut self.nodes {
      Some(nodes) => {
        for node in nodes {
          if node.insert(leaf) {
            break;
          }
        }
      }
      None => match &mut self.leafs {
        Some(leafs) => {
          if leafs.len() >= self.bucket {
            let mut nodes = subdivide(&self.boundary, self.bucket);

            for node in nodes.iter_mut() {
              if node.insert(leaf) {
                update_neighbors(&node, leaf);
                break;
              }
            }

            self.nodes = Some(nodes);
            self.leafs = None;
          } else {
            leafs.push(leaf.clone());
            update_neighbors(&self, leaf);
          }
        }
        None => {
          self.leafs = Some(vec![leaf.clone()]);
        }
      },
    }

    true
  }

  fn query(&self, range: &Boundary) -> Vec<Chunk> {
    let mut result = vec![];
    if !range.intersects(&self.boundary) {
      return result;
    }

    match &self.nodes {
      Some(nodes) => {
        for node in nodes {
          result.append(node.query(range).as_mut());
        }
      }
      None => {
        if let Some(leafs) = &self.leafs {
          for leaf in leafs {
            if range.contains(&leaf.position().into()) {
              result.push(leaf.clone())
            }
          }
        }
      }
    }

    result
  }

  fn get_leaf(&self, point: &Vector3<f32>) -> Option<Chunk> {
    if !self.boundary.contains(point) {
      return None;
    }

    match &self.nodes {
      Some(nodes) => {
        for node in nodes {
          if let Some(chunk) = node.get_leaf(point) {
            return Some(chunk);
          }
        }
      }
      None => {
        if let Some(leafs) = &self.leafs {
          for leaf in leafs {
            let position: Vector3<f32> = leaf.position().into();
            if position == *point {
              return Some(leaf.clone());
            }
          }
        }
      }
    }

    None
  }

  fn set_leaf(&mut self, leaf: &Chunk) -> bool {
    if !self.boundary.contains(&leaf.position().into()) {
      return false;
    }

    match &mut self.nodes {
      Some(nodes) => {
        for node in nodes {
          if node.set_leaf(leaf) {
            return true;
          }
        }
      }
      None => {
        if let Some(leafs) = self.leafs.as_mut() {
          for (i, old) in leafs.iter().enumerate() {
            let old_position: Vector3<f32> = old.position().into();
            let leaf_position: Vector3<f32> = leaf.position().into();
            if old_position == leaf_position {
              leafs.insert(i, leaf.clone());
              update_neighbors(&self, leaf);
              return true;
            }
          }
        }
      }
    }

    self.insert(leaf)
  }
}

// TODO: In a near future I want to use the same class to manage Quadtree and Octree
#[derive(Clone, Debug)]
pub struct Tree {
  nodes: Vec<Node>,
}

impl Tree {
  pub fn new(size: [f32; 3], bucket: usize) -> Self {
    let boundary = Boundary::new([0.0, 0.0, 0.0], size);

    Tree {
      nodes: subdivide(&boundary, bucket),
    }
  }

  pub fn insert(&mut self, leaf: &Chunk) {
    for node in self.nodes.iter_mut() {
      node.insert(leaf);
    }
  }

  pub fn query(&self, boundary: &Boundary) -> Vec<Chunk> {
    let mut result = vec![];

    for node in self.nodes.iter() {
      result.append(&mut node.query(boundary));
    }

    result
  }

  pub fn get_leaf(&self, point: &Vector3<f32>) -> Option<Chunk> {
    for node in self.nodes.iter() {
      if let Some(chunk) = node.get_leaf(point) {
        return Some(chunk);
      }
    }

    None
  }

  pub fn set_leaf(&mut self, leaf: &Chunk) -> bool {
    for node in self.nodes.iter_mut() {
      if node.set_leaf(leaf) {
        return true;
      }
    }

    false
  }
}

#[allow(clippy::many_single_char_names)]
fn subdivide(boundary: &Boundary, bucket: usize) -> Vec<Node> {
  let w = boundary.size.x / 2.0;
  let h = boundary.size.y / 2.0;
  let d = boundary.size.z / 2.0;
  let size: [f32; 3] = [w, h, d];
  let hw = size[0] / 2.0;
  let hh = size[1] / 2.0;
  let hd = size[2] / 2.0;

  let x = w - boundary.center.x;
  let y = h - boundary.center.y;
  let z = d - boundary.center.z;

  let coords: [[f32; 3]; 8] = [
    [x - hw, y + hh, z + hd],
    [x + hw, y + hh, z + hd],
    [x - hw, y + hh, z - hd],
    [x + hw, y + hh, z - hd],
    [x - hw, y - hh, z + hd],
    [x + hw, y - hh, z + hd],
    [x - hw, y - hh, z - hd],
    [x + hw, y - hh, z - hd],
  ];

  let mut result = vec![];

  for coord in coords.iter() {
    result.push(Node::new(Boundary::new(*coord, size), bucket));
  }

  result
}

fn update_neighbors(node: &Node, leaf: &Chunk) {
  let [x, y, z] = leaf.position();

  let coords: [Vector3<f32>; 6] = [
    [x - 1.0, y, z].into(),
    [x + 1.0, y, z].into(),
    [x, y - 1.0, z].into(),
    [x, y + 1.0, z].into(),
    [x, y, z - 1.0].into(),
    [x, y, z + 1.0].into(),
  ];

  for coord in coords.iter() {
    if let Some(chunk) = node.get_leaf(coord) {
      chunk.update_neighbor_data(&leaf);
    }
  }
}
