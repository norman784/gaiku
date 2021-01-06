use mint::Vector3;

pub trait Chunkify {
  fn new(position: [f32; 3], width: usize, height: usize, depth: usize) -> Self;
  fn depth(&self) -> usize;
  fn is_air(&self, x: usize, y: usize, z: usize) -> bool;
  fn get(&self, x: usize, y: usize, z: usize) -> (u8, u8);
  fn get_index(&self, x: usize, y: usize, z: usize) -> u8;
  fn get_value(&self, x: usize, y: usize, z: usize) -> u8;
  fn height(&self) -> usize;
  fn position(&self) -> Vector3<f32>;
  fn set(&mut self, x: usize, y: usize, z: usize, index_value: (u8, u8));
  fn set_index(&mut self, x: usize, y: usize, z: usize, index: u8);
  fn set_value(&mut self, x: usize, y: usize, z: usize, value: u8);
  fn width(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct Chunk {
  position: Vector3<f32>,
  width: usize,
  height: usize,
  depth: usize,
  indices_values: Vec<(u8, u8)>,
}

impl Chunk {
  fn index(&self, x: usize, y: usize, z: usize) -> usize {
    x + y * self.width + z * self.width * self.width
  }

  // TODO: This will add  the neighbor data at the border of the chunk, so we can calculate correctly  the normals, heights, etc without need to worry to query each time to get that data
  pub fn update_neighbor_data(&self, _neighbor: &Chunk) {
    unimplemented!();
  }
}

impl Chunkify for Chunk {
  fn new(position: [f32; 3], width: usize, height: usize, depth: usize) -> Self {
    Chunk {
      position: position.into(),
      width,
      height,
      depth,
      indices_values: vec![(0, 0); depth * height * width],
    }
  }

  fn depth(&self) -> usize {
    self.depth
  }

  fn is_air(&self, x: usize, y: usize, z: usize) -> bool {
    if x >= self.width || y >= self.height || z >= self.depth {
      true
    } else {
      self.get_value(x, y, z) == 0
    }
  }

  fn get(&self, x: usize, y: usize, z: usize) -> (u8, u8) {
    self.indices_values[self.index(x, y, z)]
  }

  fn get_index(&self, x: usize, y: usize, z: usize) -> u8 {
    self.get(x, y, z).0
  }

  fn get_value(&self, x: usize, y: usize, z: usize) -> u8 {
    self.get(x, y, z).1
  }

  fn height(&self) -> usize {
    self.height
  }

  fn position(&self) -> Vector3<f32> {
    self.position
  }

  fn set(&mut self, x: usize, y: usize, z: usize, value: (u8, u8)) {
    let index = self.index(x, y, z);
    self.indices_values[index] = value;
  }

  fn set_index(&mut self, x: usize, y: usize, z: usize, index: u8) {
    let value = self.get_value(x, y, z);
    self.set(x, y, z, (index, value));
  }

  fn set_value(&mut self, x: usize, y: usize, z: usize, value: u8) {
    let index = self.get_index(x, y, z);
    self.set(x, y, z, (index, value));
  }

  fn width(&self) -> usize {
    self.width
  }
}
