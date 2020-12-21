use mint::{Vector3, Vector4};

pub trait Chunkify {
  fn new(position: [f32; 3], width: usize, height: usize, depth: usize) -> Self;
  fn is_air(&self, x: usize, y: usize, z: usize) -> bool;
  fn get(&self, x: usize, y: usize, z: usize) -> u8;
  fn get_color(&self, x: usize, y: usize, z: usize) -> Option<Vector4<u8>>;
  fn index(&self, x: usize, y: usize, z: usize) -> usize;
  fn set(&mut self, x: usize, y: usize, z: usize, value: u8);
  fn set_color(&mut self, x: usize, y: usize, z: usize, color: Vector4<u8>);
}

#[derive(Debug, Clone)]
pub struct Chunk {
  colors: Vec<Vector4<u8>>,
  position: Vector3<f32>,
  width: usize,
  height: usize,
  depth: usize,
  values: Vec<u8>,
}

impl Chunk {
  pub fn colors(&self) -> &Vec<Vector4<u8>> {
    &self.colors
  }

  pub fn depth(&self) -> usize {
    self.depth
  }

  pub fn position(&self) -> &Vector3<f32> {
    &self.position
  }

  pub fn width(&self) -> usize {
    self.width
  }

  pub fn height(&self) -> usize {
    self.depth
  }

  pub fn update_neighbor_data(&mut self, _neighbor: &Chunk) {
    unimplemented!();
  }

  pub fn values(&self) -> &Vec<u8> {
    &self.values
  }
}

impl Chunkify for Chunk {
  fn new(position: [f32; 3], width: usize, height: usize, depth: usize) -> Self {
    Chunk {
      colors: vec![[0, 0, 0, 0].into(); depth * height * width],
      position: position.into(),
      width,
      height,
      depth,
      values: vec![0; depth * height * width],
    }
  }

  fn is_air(&self, x: usize, y: usize, z: usize) -> bool {
    if x >= self.width || y >= self.height || z >= self.depth {
      true
    } else {
      self.values[self.index(x, y, z)] == 0
    }
  }

  fn get(&self, x: usize, y: usize, z: usize) -> u8 {
    self.values[self.index(x, y, z)]
  }

  fn get_color(&self, x: usize, y: usize, z: usize) -> Option<Vector4<u8>> {
    let index = self.index(x, y, z);
    if let Some(color) = self.colors.get(index) {
      Some(color.clone())
    } else {
      None
    }
  }

  fn index(&self, x: usize, y: usize, z: usize) -> usize {
    get_index_from(x, y, z, self.width, self.height, self.depth)
  }

  fn set(&mut self, x: usize, y: usize, z: usize, value: u8) {
    let index = self.index(x, y, z);
    self.values[index] = value;
  }

  fn set_color(&mut self, x: usize, y: usize, z: usize, color: Vector4<u8>) {
    let index = self.index(x, y, z);
    self.colors[index] = color;
  }
}

pub fn get_index_from(
  x: usize,
  y: usize,
  z: usize,
  width: usize,
  height: usize,
  _depth: usize,
) -> usize {
  x + y * width + z * width * height
}
