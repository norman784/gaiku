#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use mint::Vector3;

pub trait Chunkify {
  fn new(position: [f32; 3], width: u16, height: u16, depth: u16) -> Self;
  fn depth(&self) -> u16;
  fn is_air(&self, x: usize, y: usize, z: usize) -> bool;
  fn get(&self, x: usize, y: usize, z: usize) -> (u8, u8);
  fn height(&self) -> u16;
  fn position(&self) -> Vector3<f32>;
  fn set(&mut self, x: usize, y: usize, z: usize, index_value: (u8, u8));
  fn width(&self) -> u16;
}

pub trait ChunkifyMut {
  fn set(&mut self, x: usize, y: usize, z: usize, index_value: (u8, u8));
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Chunk {
  position: Vector3<f32>,
  width: u16,
  height: u16,
  depth: u16,
  indices_values: Vec<(u8, u8)>,
}

impl Chunk {
  fn index(&self, x: usize, y: usize, z: usize) -> usize {
    x + y * self.width as usize + z * self.width as usize * self.width as usize
  }

  pub fn values(&self) -> Vec<(u8, u8)> {
    self.indices_values.clone()
  }

  // TODO: This will add  the neighbor data at the border of the chunk, so we can calculate correctly  the normals, heights, etc without need to worry to query each time to get that data
  pub fn update_neighbor_data(&self, _neighbor: &Chunk) {
    unimplemented!();
  }
}

impl Chunkify for Chunk {
  fn new(position: [f32; 3], width: u16, height: u16, depth: u16) -> Self {
    Chunk {
      position: position.into(),
      width,
      height,
      depth,
      indices_values: vec![(0, 0); depth as usize * height as usize * width as usize],
    }
  }

  fn depth(&self) -> u16 {
    self.depth
  }

  fn is_air(&self, x: usize, y: usize, z: usize) -> bool {
    if x >= self.width as usize || y >= self.height as usize || z >= self.depth as usize {
      true
    } else {
      self.get(x, y, z).1 == 0
    }
  }

  fn get(&self, x: usize, y: usize, z: usize) -> (u8, u8) {
    self.indices_values[self.index(x, y, z)]
  }

  fn height(&self) -> u16 {
    self.height
  }

  fn position(&self) -> Vector3<f32> {
    self.position
  }

  fn set(&mut self, x: usize, y: usize, z: usize, value: (u8, u8)) {
    let index = self.index(x, y, z);
    self.indices_values[index] = value;
  }

  fn width(&self) -> u16 {
    self.width
  }
}
