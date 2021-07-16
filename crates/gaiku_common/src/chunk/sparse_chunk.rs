#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::{
  atlas::{Atlasify, AtlasifyMut},
  boxify::*,
  chunk::{Chunkify, ChunkifyMut},
};

/// Provides a `Chunkify` implementation with a hashmap and `u8` position based on x, y and z axis with `u8` value.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SparseChunk {
  width: u16,
  height: u16,
  depth: u16,
  data: HashMap<(usize, usize, usize), (u8, f32)>,
}

impl Chunkify<f32> for SparseChunk {
  fn is_air(&self, x: usize, y: usize, z: usize, isovalue: f32) -> bool {
    self.get(x, y, z) - isovalue < 1e-4
  }

  fn get(&self, x: usize, y: usize, z: usize) -> f32 {
    self.data.get(&(x, y, z)).map(|d| d.1).unwrap_or(-1.)
  }
}

impl Atlasify<u8> for SparseChunk {
  fn get_atlas(&self, x: usize, y: usize, z: usize) -> u8 {
    self.data.get(&(x, y, z)).map(|d| d.0).unwrap_or(0)
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

impl ChunkifyMut<f32> for SparseChunk {
  fn set(&mut self, x: usize, y: usize, z: usize, value: f32) {
    let atlas = self.get_atlas(x, y, z);
    self.data.insert((x, y, z), (atlas, value));
  }
}

impl AtlasifyMut<u8> for SparseChunk {
  fn set_atlas(&mut self, x: usize, y: usize, z: usize, atlas: u8) {
    let value = self.get(x, y, z);
    self.data.insert((x, y, z), (atlas, value));
  }
}
