use std::time::Instant;

use gaiku_baker_voxel::VoxelBaker;
use gaiku_common::{
  chunk::Chunk,
  mesh::Mesh,
  prelude::*,
  texture::{Texture2d, TextureAtlas2d},
  Result,
};
use gaiku_format_gox::GoxReader;

mod common;

use crate::common::export;

fn read(name: &str) -> Result<()> {
  let now = Instant::now();
  let file = format!(
    "{}/examples/assets/{}.gox",
    env!("CARGO_MANIFEST_DIR"),
    name
  );
  let (chunks, texture): (Vec<Chunk>, Option<TextureAtlas2d<Texture2d>>) = GoxReader::read(&file)?;
  let options = BakerOptions {
    texture,
    ..Default::default()
  };
  let mut meshes: Vec<(Mesh, [f32; 3])> = vec![];

  let reader_elapsed = now.elapsed().as_micros();
  let now = Instant::now();

  for chunk in chunks.iter() {
    let mesh = VoxelBaker::bake(chunk, &options)?;
    if let Some(mesh) = mesh {
      meshes.push((mesh, chunk.position()));
    }
  }

  let baker_elapsed = now.elapsed().as_micros();
  let now = Instant::now();

  export(meshes, &format!("{}_vx", name));

  println!(
    "<<{}>> Chunks: {} Reader: {} micros Baker: {} micros Export: {} micros",
    name,
    chunks.len(),
    reader_elapsed,
    baker_elapsed,
    now.elapsed().as_micros()
  );

  Ok(())
}

fn main() -> Result<()> {
  let _ = read("small_tree");
  let _ = read("terrain");
  let _ = read("planet");

  Ok(())
}
