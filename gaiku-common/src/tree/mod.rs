use nalgebra::Point3;
use crate::data::Chunk;

pub type Octree = Tree;

#[derive(Clone, Debug, new)]
pub struct Boundary {
  center: Point3<f64>,
  size: Point3<f64>,
}

impl Boundary {
  fn contains(&self, point: &Point3<f64>) -> bool {
    self.start_x() > point.coords[0] &&
        self.start_y() > point.coords[1] &&
        self.start_z() > point.coords[2] &&
        self.end_x() < point.coords[0] &&
        self.end_y() < point.coords[1] &&
        self.end_z() < point.coords[2]
  }

  fn intersects(&self, range: &Boundary) -> bool {
    !(
      range.start_x() > self.start_x() ||
          range.start_y() > self.start_y() ||
          range.start_z() > self.start_z() ||
          range.end_x() < self.end_x() ||
          range.end_y() < self.end_y() ||
          range.end_z() < self.end_z()
    )
  }

  fn start_x(&self) -> f64 {
    self.center.coords[0] - self.size.coords[0]
  }

  fn start_y(&self) -> f64 {
    self.center.coords[1] - self.size.coords[1]
  }

  fn start_z(&self) -> f64 {
    self.center.coords[2] - self.size.coords[2]
  }

  fn end_x(&self) -> f64 {
    self.center.coords[0] + self.size.coords[0]
  }

  fn end_y(&self) -> f64 {
    self.center.coords[1] + self.size.coords[1]
  }

  fn end_z(&self) -> f64 {
    self.center.coords[2] + self.size.coords[2]
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
      return false
    }

    match &mut self.nodes {
      Some(nodes) => {
        for node in nodes {
          if node.insert(leaf) {
            break;
          }
        }
      }
      None => {
        match &mut self.leafs {
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
        }
      }
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
        match &self.leafs {
          Some(leafs) => {
            for leaf in leafs {
              if range.contains(&leaf.position()) {
                result.push(leaf.clone())
              }
            }
          }
          _ => {}
        }
      }
    }

    result
  }

  fn get_leaf(&self, point: &Point3<f64>) -> Option<Chunk> {
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
        match &self.leafs {
          Some(leafs) => {
            for leaf in leafs {
              if leaf.position() == point {
                return Some(leaf.clone())
              }
            }
          }
          _ => {}
        }
      }
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
      None => {
        match &mut self.leafs {
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
  pub fn new(size: Point3<f64>, bucket: usize) -> Self {
    let boundary = Boundary::new([0.0, 0.0, 0.0].into(), size);
    
    Tree { 
      nodes: subdivide(&boundary, bucket)
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

  pub fn get_leaf(&self, point: &Point3<f64>) -> Option<Chunk> {
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
  let w = boundary.size.coords[0] / 2.0;
  let h = boundary.size.coords[1] / 2.0;
  let d = boundary.size.coords[2] / 2.0;
  let node_size: Point3<f64> = [w, h, d].into();
  let hw = node_size.coords[0] / 2.0;
  let hh = node_size.coords[1] / 2.0;
  let hd = node_size.coords[2] / 2.0;

  let x = w - boundary.center.coords[0];
  let y = h - boundary.center.coords[1];
  let z = d - boundary.center.coords[2];

  let coords: [Point3<f64>; 8] = [
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
    result.push(Node::new(Boundary::new(node_size, coord.clone()), bucket));
  }

  result
}

fn update_neighbors(node: &Node, leaf: &Chunk) {
  let x = leaf.position().coords[0];
  let y = leaf.position().coords[1];
  let z = leaf.position().coords[2];

  let coords: [Point3<f64>; 6] = [
    [x - 1.0, y, z].into(),
    [x + 1.0, y, z].into(),
    [x, y - 1.0, z].into(),
    [x, y + 1.0, z].into(),
    [x, y, z - 1.0].into(),
    [x, y, z + 1.0].into(),
  ];

  for coord in  coords.iter() {
    if let Some(chunk) = node.get_leaf(coord) {
      chunk.update_neighbor_data(&leaf);
    }
  }
}
