use crate::{Color, Vector3};

#[derive(Debug, Clone)]
pub struct Chunk {
  colors: Vec<Color>,
  position: Vector3,
  width: usize,
  height: usize,
  depth: usize,
  values: Vec<u8>,
}

impl Chunk {
  pub fn new(position: Vector3, width: usize, height: usize, depth: usize) -> Self {
    Chunk {
      colors: vec![[0, 0, 0, 0]; depth * height * width],
      position,
      width,
      height,
      depth,
      values: vec![0; depth * height * width],
    }
  }

  pub fn depth(&self) -> usize {
    self.depth
  }

  pub fn get(&self, x: usize, y: usize, z: usize) -> u8 {
    self.values[get_index_for(&self, x, y, z)]
  }

  pub fn get_color(&self, x: usize, y: usize, z: usize) -> Option<Color> {
    let index = get_index_for(&self, x, y, z);
    if let Some(color) = self.colors.get(index) {
      Some(color.clone())
    } else {
      None
    }
  }

  pub fn height(&self) -> usize {
    self.height
  }

  pub fn is_empty(&self, x: usize, y: usize, z: usize) -> bool {
    if x >= self.width || y >= self.height || z >= self.depth {
      true
    } else {
      self.values[get_index_for(&self, x, y, z)] == 0
    }
  }

  pub fn position(&self) -> Vector3 {
    self.position
  }

  pub fn set(&mut self, x: usize, y: usize, z: usize, value: u8) {
    let index = get_index_for(&self, x, y, z);
    self.values[index] = value;
  }

  pub fn set_color(&mut self, x: usize, y: usize, z: usize, color: Color) {
    let index = get_index_for(&self, x, y, z);
    self.colors[index] = color;
  }

  // TODO: This will add  the neighbor data at the border of the chunk, so we can calculate correctly  the normals, heights, etc without need to worry to query each time to get that data
  pub fn update_neighbor_data(&self, _neighbor: &Chunk) {
    unimplemented!();
  }

  pub fn width(&self) -> usize {
    self.width
  }
}

pub fn get_index_for(chunk: &Chunk, x: usize, y: usize, z: usize) -> usize {
  x + y * chunk.width + z * chunk.width * chunk.height
}
