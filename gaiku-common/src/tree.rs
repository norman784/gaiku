use crate::data::{Chunk, Chunkify};
use mint::Vector3;

pub type Octree = Tree;

#[derive(Clone, Debug)]
pub struct Boundary {
  center: Vector3<f32>,
  size: Vector3<f32>,
}

impl Boundary {
  fn contains(&self, point: &Vector3<f32>) -> bool {
    self.start_x() > point.x
      && self.start_y() > point.y
      && self.start_z() > point.z
      && self.end_x() < point.x
      && self.end_y() < point.y
      && self.end_z() < point.z
  }

  fn intersects(&self, range: &Boundary) -> bool {
    !(range.start_x() > self.start_x()
      || range.start_y() > self.start_y()
      || range.start_z() > self.start_z()
      || range.end_x() < self.end_x()
      || range.end_y() < self.end_y()
      || range.end_z() < self.end_z())
  }

  fn start_x(&self) -> f32 {
    self.center.x - self.size.x
  }

  fn start_y(&self) -> f32 {
    self.center.y - self.size.y
  }

  fn start_z(&self) -> f32 {
    self.center.z - self.size.z
  }

  fn end_x(&self) -> f32 {
    self.center.x + self.size.x
  }

  fn end_y(&self) -> f32 {
    self.center.y + self.size.y
  }

  fn end_z(&self) -> f32 {
    self.center.z + self.size.z
  }
}

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
    if !self.boundary.contains(&leaf.position()) {
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
      None => match &self.leafs {
        Some(leafs) => {
          for leaf in leafs {
            if range.contains(&leaf.position()) {
              result.push(leaf.clone())
            }
          }
        }
        _ => {}
      },
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
      None => match &self.leafs {
        Some(leafs) => {
          for leaf in leafs {
            if &leaf.position() == point {
              return Some(leaf.clone());
            }
          }
        }
        _ => {}
      },
    }

    None
  }

  fn set_leaf(&mut self, leaf: &Chunk) -> bool {
    if !self.boundary.contains(&leaf.position()) {
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
      None => match &mut self.leafs {
        Some(leafs) => {
          for (i, old) in leafs.iter().enumerate() {
            if old.position() == leaf.position() {
              leafs.insert(i, leaf.clone());
              update_neighbors(&self, leaf);
              return true;
            }
          }
        }
        _ => {}
      },
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
  pub fn new(size: Vector3<f32>, bucket: usize) -> Self {
    let boundary = Boundary {
      center: [0.0, 0.0, 0.0].into(),
      size,
    };

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

fn subdivide(boundary: &Boundary, bucket: usize) -> Vec<Node> {
  let w = boundary.size.x / 2.0;
  let h = boundary.size.y / 2.0;
  let d = boundary.size.z / 2.0;
  let size: Vector3<f32> = [w, h, d].into();
  let hw = size.x / 2.0;
  let hh = size.y / 2.0;
  let hd = size.z / 2.0;

  let x = w - boundary.center.x;
  let y = h - boundary.center.y;
  let z = d - boundary.center.z;

  let coords: [Vector3<f32>; 8] = [
    [x - hw, y + hh, z + hd].into(),
    [x + hw, y + hh, z + hd].into(),
    [x - hw, y + hh, z - hd].into(),
    [x + hw, y + hh, z - hd].into(),
    [x - hw, y - hh, z + hd].into(),
    [x + hw, y - hh, z + hd].into(),
    [x - hw, y - hh, z - hd].into(),
    [x + hw, y - hh, z - hd].into(),
  ];

  let mut result = vec![];

  for coord in coords.iter() {
    result.push(Node::new(
      Boundary {
        center: coord.clone(),
        size,
      },
      bucket,
    ));
  }

  result
}

fn update_neighbors(node: &Node, leaf: &Chunk) {
  let x = leaf.position().x;
  let y = leaf.position().y;
  let z = leaf.position().z;

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
