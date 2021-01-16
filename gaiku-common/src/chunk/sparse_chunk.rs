#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::{
  boxify::*,
  chunk::{Chunkify, ChunkifyMut},
};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SparseChunk {
  width: u16,
  height: u16,
  depth: u16,
  data: HashMap<(usize, usize, usize), u8>,
}

impl Chunkify<u8> for SparseChunk {
  fn is_air(&self, x: usize, y: usize, z: usize) -> bool {
    self.get(x, y, z) == 0
  }

  fn get(&self, x: usize, y: usize, z: usize) -> u8 {
    *self.data.get(&(x, y, z)).unwrap_or(&0)
  }
}

impl Sizable for SparseChunk {
  fn with_size(width: u16, height: u16, depth: u16) -> Self {
    Self {
      width,
      height,
      depth,
      data: HashMap::new(),
    }
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

impl ChunkifyMut<u8> for SparseChunk {
  fn set(&mut self, x: usize, y: usize, z: usize, value: u8) {
    self.data.insert((x, y, z), value);
  }
}
