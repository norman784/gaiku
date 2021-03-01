#![feature(test)]

extern crate test;

use gaiku::{
  common::{
    chunk::Chunk,
    mesh::Mesh,
    prelude::*,
    texture::{Texture2d, TextureAtlas2d},
    Result,
  },
  GoxReader, HeightMapBaker,
};
use test::Bencher;

fn get_chunks(name: &str) -> Result<(Vec<Chunk>, Option<TextureAtlas2d<Texture2d>>)> {
  let file = format!(
    "{}/examples/assets/{}.gox",
    env!("CARGO_MANIFEST_DIR"),
    name
  );

  GoxReader::read(&file)
}

#[bench]
fn heightmap_terrain(b: &mut Bencher) -> Result<()> {
  let (chunks, texture) = get_chunks("terrain").unwrap();
  let options = BakerOptions {
    texture,
    ..Default::default()
  };

  b.iter(|| {
    let mut meshes: Vec<(Mesh, [f32; 3])> = vec![];

    for chunk in chunks.iter() {
      let mesh = HeightMapBaker::bake(chunk, &options).unwrap();
      if let Some(mesh) = mesh {
        meshes.push((mesh, chunk.position()));
      }
    }
  });

  Ok(())
}

#[bench]
fn heightmap_planet(b: &mut Bencher) -> Result<()> {
  let (chunks, texture) = get_chunks("planet").unwrap();
  let options = BakerOptions {
    texture,
    ..Default::default()
  };

  b.iter(|| {
    let mut meshes: Vec<(Mesh, [f32; 3])> = vec![];

    for chunk in chunks.iter() {
      let mesh = HeightMapBaker::bake(chunk, &options).unwrap();
      if let Some(mesh) = mesh {
        meshes.push((mesh, chunk.position()));
      }
    }
  });

  Ok(())
}

#[bench]
fn heightmap_small_tree(b: &mut Bencher) -> Result<()> {
  let (chunks, texture) = get_chunks("small_tree").unwrap();
  let options = BakerOptions {
    texture,
    ..Default::default()
  };

  b.iter(|| {
    let mut meshes: Vec<(Mesh, [f32; 3])> = vec![];

    for chunk in chunks.iter() {
      let mesh = HeightMapBaker::bake(chunk, &options).unwrap();
      if let Some(mesh) = mesh {
        meshes.push((mesh, chunk.position()));
      }
    }
  });

  Ok(())
}

#[bench]
fn heightmap_small_checkerboard(b: &mut Bencher) -> Result<()> {
  let width: usize = 3;
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
          chunk.set(x, y, z, (1, 1));
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
    HeightMapBaker::bake::<Chunk, Texture2d, Mesh>(&chunk, &options).unwrap();
  });

  Ok(())
}

#[bench]
fn heightmap_medium_checkerboard(b: &mut Bencher) -> Result<()> {
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
          chunk.set(x, y, z, (1, 1));
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
    HeightMapBaker::bake::<Chunk, Texture2d, Mesh>(&chunk, &options).unwrap();
  });

  Ok(())
}

#[bench]
fn heightmap_large_checkerboard(b: &mut Bencher) -> Result<()> {
  let width: usize = 30;
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
          chunk.set(x, y, z, (1, 1));
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
    HeightMapBaker::bake::<Chunk, Texture2d, Mesh>(&chunk, &options).unwrap();
  });

  Ok(())
}
