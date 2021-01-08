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

  let reader_elapsed = now.elapsed().as_secs();
  let now = Instant::now();

  for chunk in chunks.iter() {
    let mesh = VoxelBaker::bake(chunk, &options)?;
    if let Some(mesh) = mesh {
      meshes.push((mesh, chunk.position()));
    }
  }

  let baker_elapsed = now.elapsed().as_secs();
  let now = Instant::now();

  if let Some(texture) = options.texture {
    println!("TEXTURE::::::");
    println!("{:?}", texture.get_texture().get_data());
    texture.get_texture().write_to_file(&format!(
      "{}/examples/output/{}.png",
      env!["CARGO_MANIFEST_DIR"],
      name
    ))?;
  }
  export(meshes, &format!("{}_vx", name));

  println!(
    "<<{}>> Chunks: {} Reader: {} Baker: {} secs Export: {} secs",
    name,
    chunks.len(),
    reader_elapsed,
    baker_elapsed,
    now.elapsed().as_secs()
  );

  Ok(())
}

fn main() -> Result<()> {
  let _ = read("small_tree");
  //let _ = read("terrain");
  //let _ = read("planet");

  Ok(())
}
