use mint::{Vector3, Vector4};

pub trait Chunkify {
  fn new(position: [f32; 3], width: usize, height: usize, depth: usize) -> Self;
  fn is_air(&self, x: usize, y: usize, z: usize) -> bool;
  fn get(&self, x: usize, y: usize, z: usize) -> u8;
  fn set(&mut self, x: usize, y: usize, z: usize, value: u8);
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

  pub fn get_with_color(&self, x: usize, y: usize, z: usize) -> (u8, Vector4<u8>) {
    if let Some(index) = self.index(x, y, z) {
      (self.values[index], self.colors[index])
    } else {
      (0, [0, 0, 0, 0].into())
    }
  }

  fn index(&self, x: usize, y: usize, z: usize) -> Option<usize> {
    get_index_from(x, y, z, self.width, self.height, self.depth)
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

  pub fn set_with_color(&mut self, x: usize, y: usize, z: usize, value: u8, color: Vector4<u8>) {
    if let Some(index) = self.index(x, y, z) {
      self.values[index] = value;
      self.colors[index] = color;
    }
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
    if let Some(index) = self.index(x, y, z) {
      self.values[index] == 0
    } else {
      true
    }
  }

  fn get(&self, x: usize, y: usize, z: usize) -> u8 {
    if let Some(index) = self.index(x, y, z) {
      self.values[index]
    } else {
      0
    }
  }

  fn set(&mut self, x: usize, y: usize, z: usize, value: u8) {
    let color = if self.get_with_color(x, y, z).1.w == 0 {
      [255, 255, 255, 255]
    } else {
      [0, 0, 0, 0]
    };

    self.set_with_color(x, y, z, value, color.into());
  }
}

pub fn get_index_from(
  x: usize,
  y: usize,
  z: usize,
  width: usize,
  height: usize,
  deepth: usize,
) -> Option<usize> {
  if x < width && y < height && z < deepth {
    Some(x + y * width + z * width * height)
  } else {
    None
  }
}
