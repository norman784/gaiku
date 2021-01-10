use std::time::Instant;

use gaiku_3d::{
  bakers::VoxelBaker,
  common::{Baker, BakerOptions, Chunkify, FileFormat, Result},
  formats::GoxReader,
};

mod common;

use crate::common::export;

fn read(name: &str) -> Result<()> {
  let now = Instant::now();
  let file = format!(
    "{}/examples/assets/{}.gox",
    env!("CARGO_MANIFEST_DIR"),
    name
  );
  let (chunks, texture) = GoxReader::read(&file)?;
  let options = BakerOptions {
    texture,
    ..Default::default()
  };
  let mut meshes = vec![];

  let reader_elapsed = now.elapsed().as_micros();
  let now = Instant::now();

  for chunk in chunks.iter() {
    let mesh = VoxelBaker::bake(chunk, &options)?;
    if let Some(mesh) = mesh {
      meshes.push((mesh, chunk.position().into()));
    }
  }

  let baker_elapsed = now.elapsed().as_micros();
  let now = Instant::now();

  if let Some(texture) = options.texture {
    texture.get_texture().write_to_file(&format!(
      "{}/examples/output/{}.png",
      env!["CARGO_MANIFEST_DIR"],
      name
    ))?;
  }

  let mesh_count = meshes.len();
  meshes
    .iter()
    .for_each(|(m, _)| println!("Mesh {}", m.indices.as_ref().unwrap().len()));

  export(meshes, &format!("{}_vx", name));

  println!(
    "<<{}>> Chunks: {} Meshes: {} Reader: {} micros Baker: {} micros Export: {} micros",
    name,
    chunks.len(),
    mesh_count,
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
