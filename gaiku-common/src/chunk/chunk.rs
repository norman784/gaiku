#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::{
  boxify::*,
  chunk::{Chunkify, ChunkifyMut},
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Chunk {
  position: [f32; 3],
  width: u16,
  height: u16,
  depth: u16,
  values: Vec<(u8, u8)>,
}

impl Chunk {
  fn index(&self, x: usize, y: usize, z: usize) -> usize {
    x + y * self.width as usize + z * self.width as usize * self.height as usize
  }

  pub fn values(&self) -> Vec<(u8, u8)> {
    self.values.clone()
  }

  // TODO: This will add  the neighbor data at the border of the chunk, so we can calculate correctly  the normals, heights, etc without need to worry to query each time to get that data
  pub fn update_neighbor_data(&self, _neighbor: &Chunk) {
    unimplemented!();
  }
}

impl Boxify for Chunk {
  fn new(position: [f32; 3], width: u16, height: u16, depth: u16) -> Self {
    Self {
      position,
      width,
      height,
      depth,
      values: vec![(0, 0); depth as usize * height as usize * width as usize],
    }
  }
}

impl Chunkify<(u8, u8)> for Chunk {
  fn is_air(&self, x: usize, y: usize, z: usize) -> bool {
    if x >= self.width as usize || y >= self.height as usize || z >= self.depth as usize {
      true
    } else {
      self.get(x, y, z).1 == 0
    }
  }

  fn get(&self, x: usize, y: usize, z: usize) -> (u8, u8) {
    self.values[self.index(x, y, z)]
  }
}

impl ChunkifyMut<(u8, u8)> for Chunk {
  fn set(&mut self, x: usize, y: usize, z: usize, value: (u8, u8)) {
    let index = self.index(x, y, z);
    self.values[index] = value;
  }
}

impl Positionable for Chunk {
  fn with_position(position: [f32; 3]) -> Self {
    Self::new(position, 16, 16, 16)
  }

  fn position(&self) -> [f32; 3] {
    self.position
  }
}

impl Sizable for Chunk {
  fn with_size(width: u16, height: u16, depth: u16) -> Self {
    Self::new([0.0, 0.0, 0.0], width, height, depth)
  }

  fn depth(&self) -> u16 {
    self.depth
  }

  fn height(&self) -> u16 {
    self.height
  }

  fn width(&self) -> u16 {
    self.width
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn check_index() {
    let chunk = Chunk::new([0.0, 0.0, 0.0], 4, 4, 4);
    let index = chunk.index(1, 2, 3);
    assert_eq!(index, 57);

    let chunk = Chunk::new([0.0, 0.0, 0.0], 4, 5, 6);
    let index = chunk.index(1, 2, 3);
    assert_eq!(index, 69);
  }
}
