//! `gaiku-common` contains the core componets used across all the gaiku crates.
//!
//! The idea behind this crate is to offer a common interop interface to
//! work with different file formats and mesh generators, based on voxels.
use std::fs::read;

pub use anyhow::Result;
pub use mint;

use crate::{
  atlas::{Atlasify, AtlasifyMut},
  boxify::*,
  chunk::{Chunkify, ChunkifyMut},
  mesh::Meshify,
  texture::{TextureAtlas2d, Texturify2d},
};

// Traits involving the atlas
mod atlas;
// Boundary structure for octree
mod boundary;
/// Trait to define position and size.
pub mod boxify;
/// Chunk implementation, also offers all traits used internally to build the chunk object.
pub mod chunk;
/// Mesh related traits/implementation, also offers some utils like MeshBuilder.
pub mod mesh;
/// Texture related traits/implementation.
pub mod texture;
//mod tree;

/// `use gaiku_common::prelude::*;` to import common traits and utils.
pub mod prelude {
  pub use crate::{
    atlas::{Atlasify, AtlasifyMut},
    boxify::*,
    chunk::{Chunkify, ChunkifyMut},
    mesh::{MeshBuilder, Meshify},
    texture::{TextureAtlas2d, Texturify2d},
    Baker, BakerOptions, FileFormat,
  };
}

/// Options to customize the `Baker` behaviour
pub struct BakerOptions<T>
where
  T: Texturify2d,
{
  pub isovalue: f32,
  pub level_of_detail: usize,
  pub texture: Option<TextureAtlas2d<T>>,
}

impl<T> Default for BakerOptions<T>
where
  T: Texturify2d,
{
  fn default() -> Self {
    Self {
      isovalue: 0.,
      level_of_detail: 1,
      texture: None,
    }
  }
}

/// Baker is a trait used to define a chunk to mesh converter
pub trait Baker {
  type Value;
  type AtlasValue;

  fn bake<C, T, M>(chunk: &C, options: &BakerOptions<T>) -> Result<Option<M>>
  where
    C: Chunkify<Self::Value> + Atlasify<Self::AtlasValue> + Sizable,
    T: Texturify2d,
    M: Meshify;
}

/// FileFormat is a trait used to define a {file extension} to chunk converter
pub trait FileFormat {
  type Value;
  type AtlasValue;

  fn load<C, T>(bytes: Vec<u8>) -> Result<(Vec<C>, Option<TextureAtlas2d<T>>)>
  where
    C: Chunkify<Self::Value> + ChunkifyMut<Self::Value> + AtlasifyMut<Self::AtlasValue> + Boxify,
    T: Texturify2d;

  fn read<C, T>(file: &str) -> Result<(Vec<C>, Option<TextureAtlas2d<T>>)>
  where
    C: Chunkify<Self::Value> + ChunkifyMut<Self::Value> + AtlasifyMut<Self::AtlasValue> + Boxify,
    T: Texturify2d,
  {
    let bytes = read(file)?;
    Self::load::<C, T>(bytes)
  }
}

/*
pub struct Gaiku {
  terrain: Octree,
}

impl Gaiku {
  pub fn new(data: Vec<Chunk>, size: [f32; 3]) -> Self {
    let mut terrain = Octree::new(size, 8);

    for chunk in data {
      terrain.insert(&chunk);
    }

    Self { terrain }
  }

  pub fn query(&self, boundary: &Boundary) -> Vec<Chunk> {
    self.terrain.query(boundary)
  }

  pub fn get_chunk(&self, point: &Vector3<f32>) -> Option<Chunk> {
    self.terrain.get_leaf(point)
  }

  pub fn set_chunk(&mut self, chunk: &Chunk) -> bool {
    self.terrain.set_leaf(chunk)
  }
}
*/
