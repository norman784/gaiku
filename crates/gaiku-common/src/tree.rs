use crate::{data::Chunk, Vector3};

#[derive(Clone, Debug)]
pub struct Boundary {
  center: Vector3,
  size: Vector3,
}

impl Boundary {
  pub fn new(center: Vector3, size: Vector3) -> Self {
    Self { center, size }
  }

  pub fn contains(&self, point: &Vector3) -> bool {
    self.start_x() > point[0]
      && self.start_y() > point[1]
      && self.start_z() > point[2]
      && self.end_x() < point[0]
      && self.end_y() < point[1]
      && self.end_z() < point[2]
  }

  pub fn intersects(&self, range: &Boundary) -> bool {
    !(range.start_x() > self.start_x()
      || range.start_y() > self.start_y()
      || range.start_z() > self.start_z()
      || range.end_x() < self.end_x()
      || range.end_y() < self.end_y()
      || range.end_z() < self.end_z())
  }

  fn start(&self, pos: usize) -> f32 {
    self.center[pos] - self.size[pos]
  }

  pub fn start_x(&self) -> f32 {
    self.start(0)
  }

  pub fn start_y(&self) -> f32 {
    self.start(1)
  }

  pub fn start_z(&self) -> f32 {
    self.start(2)
  }

  fn end(&self, pos: usize) -> f32 {
    self.center[pos] + self.size[pos]
  }

  pub fn end_x(&self) -> f32 {
    self.end(0)
  }

  pub fn end_y(&self) -> f32 {
    self.end(1)
  }

  pub fn end_z(&self) -> f32 {
    self.end(2)
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

  fn get_leaf(&self, point: &Vector3) -> Option<Chunk> {
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
            if leaf.position() == *point {
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

#[derive(Clone, Debug)]
pub struct Octree {
  nodes: Vec<Node>,
  size: Vector3,
}

impl Octree {
  pub fn new(size: Vector3, bucket: usize) -> Self {
    let boundary = Boundary::new([0.0, 0.0, 0.0], size);

    Octree {
      nodes: subdivide(&boundary, bucket),
      size,
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

  pub fn size(&self) -> Vector3 {
    self.size
  }

  pub fn get_leaf(&self, point: &Vector3) -> Option<Chunk> {
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
  let w = boundary.size[0] / 2.0;
  let h = boundary.size[1] / 2.0;
  let d = boundary.size[2] / 2.0;
  let node_size: Vector3 = [w, h, d];
  let hw = node_size[0] / 2.0;
  let hh = node_size[1] / 2.0;
  let hd = node_size[2] / 2.0;

  let x = w - boundary.center[0];
  let y = h - boundary.center[1];
  let z = d - boundary.center[2];

  let coords: [Vector3; 8] = [
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
    result.push(Node::new(Boundary::new(node_size, coord.clone()), bucket));
  }

  result
}

fn update_neighbors(node: &Node, leaf: &Chunk) {
  let [x, y, z] = leaf.position();

  let coords: [Vector3; 6] = [
    [x - 1.0, y, z],
    [x + 1.0, y, z],
    [x, y - 1.0, z],
    [x, y + 1.0, z],
    [x, y, z - 1.0],
    [x, y, z + 1.0],
  ];

  for coord in coords.iter() {
    if let Some(chunk) = node.get_leaf(coord) {
      chunk.update_neighbor_data(&leaf);
    }
  }
}
