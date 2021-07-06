#![feature(test)]

extern crate test;
use test::Bencher;

use gaiku::{
  common::{
    chunk::Chunk,
    mesh::Mesh,
    prelude::*,
    texture::{Texture2d, TextureAtlas2d},
    Result,
  },
  VoxelBaker,
};

#[bench]
#[allow(clippy::unnecessary_wraps)]
fn meshbuilder_notree(b: &mut Bencher) -> Result<()> {
  let width: usize = 10;
  let height: usize = width;
  let depth: usize = width;
  let mut chunk = Chunk::new([0., 0., 0.], width as u16, height as u16, depth as u16);

  for x in 0..width {
    let x_fill = (x % 2) == 0;
    for y in 0..height {
      let y_fill = (y % 2) == 0;
      for z in 0..depth {
        let z_fill = (z % 2) == 0;
        if (x_fill ^ y_fill) ^ z_fill {
          // Chunk where every other voxel is set like a 3d checkerboard
          chunk.set(x, y, z, 1.);
        }
      }
    }
  }

  let atlas = TextureAtlas2d::<Texture2d>::new(1);
  let options = BakerOptions {
    texture: Some(atlas),
    ..Default::default()
  };

  b.iter(|| {
    VoxelBaker::bake_with_builder::<Chunk, Texture2d, Mesh, NoTreeBuilder>(
      &chunk,
      &options,
      Default::default(),
    )
    .unwrap();
  });

  Ok(())
}

#[bench]
#[allow(clippy::unnecessary_wraps)]
fn meshbuilder_octree(b: &mut Bencher) -> Result<()> {
  let width: usize = 10;
  let height: usize = width;
  let depth: usize = width;
  let mut chunk = Chunk::new([0., 0., 0.], width as u16, height as u16, depth as u16);

  for x in 0..width {
    let x_fill = (x % 2) == 0;
    for y in 0..height {
      let y_fill = (y % 2) == 0;
      for z in 0..depth {
        let z_fill = (z % 2) == 0;
        if (x_fill ^ y_fill) ^ z_fill {
          // Chunk where every other voxel is set like a 3d checkerboard
          chunk.set(x, y, z, 1.);
        }
      }
    }
  }

  let atlas = TextureAtlas2d::<Texture2d>::new(1);
  let options = BakerOptions {
    texture: Some(atlas),
    ..Default::default()
  };

  b.iter(|| {
    VoxelBaker::bake_with_builder::<Chunk, Texture2d, Mesh, OctMeshBuilder>(
      &chunk,
      &options,
      Default::default(),
    )
    .unwrap();
  });

  Ok(())
}

#[bench]
#[allow(clippy::unnecessary_wraps)]
fn meshbuilder_rstar(b: &mut Bencher) -> Result<()> {
  let width: usize = 10;
  let height: usize = width;
  let depth: usize = width;
  let mut chunk = Chunk::new([0., 0., 0.], width as u16, height as u16, depth as u16);

  for x in 0..width {
    let x_fill = (x % 2) == 0;
    for y in 0..height {
      let y_fill = (y % 2) == 0;
      for z in 0..depth {
        let z_fill = (z % 2) == 0;
        if (x_fill ^ y_fill) ^ z_fill {
          // Chunk where every other voxel is set like a 3d checkerboard
          chunk.set(x, y, z, 1.);
        }
      }
    }
  }

  let atlas = TextureAtlas2d::<Texture2d>::new(1);
  let options = BakerOptions {
    texture: Some(atlas),
    ..Default::default()
  };

  b.iter(|| {
    VoxelBaker::bake_with_builder::<Chunk, Texture2d, Mesh, RstarMeshBuilder>(
      &chunk,
      &options,
      Default::default(),
    )
    .unwrap();
  });

  Ok(())
}

#[bench]
#[allow(clippy::unnecessary_wraps)]
fn meshbuilder_hashmapbuilder(b: &mut Bencher) -> Result<()> {
  let width: usize = 10;
  let height: usize = width;
  let depth: usize = width;
  let mut chunk = Chunk::new([0., 0., 0.], width as u16, height as u16, depth as u16);

  for x in 0..width {
    let x_fill = (x % 2) == 0;
    for y in 0..height {
      let y_fill = (y % 2) == 0;
      for z in 0..depth {
        let z_fill = (z % 2) == 0;
        if (x_fill ^ y_fill) ^ z_fill {
          // Chunk where every other voxel is set like a 3d checkerboard
          chunk.set(x, y, z, 1.);
        }
      }
    }
  }

  let atlas = TextureAtlas2d::<Texture2d>::new(1);
  let options = BakerOptions {
    texture: Some(atlas),
    ..Default::default()
  };

  b.iter(|| {
    VoxelBaker::bake_with_builder::<Chunk, Texture2d, Mesh, HashMapBuilder>(
      &chunk,
      &options,
      Default::default(),
    )
    .unwrap();
  });

  Ok(())
}
