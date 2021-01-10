use std::time::Instant;

use gaiku_3d::{
  bakers::HeightMapBaker,
  common::{prelude::*, Result},
  formats::PNGReader,
};

mod common;

use crate::common::export;

fn read(name: &str) -> Result<()> {
  let now = Instant::now();
  let file = format!(
    "{}/examples/assets/{}.png",
    env!("CARGO_MANIFEST_DIR"),
    name
  );
  let (chunks, texture) = PNGReader::read(&file)?;
  let options = BakerOptions {
    texture,
    ..Default::default()
  };
  let mut meshes = vec![];

  let reader_elapsed = now.elapsed().as_micros();
  let now = Instant::now();

  for chunk in chunks.iter() {
    let mesh = HeightMapBaker::bake(chunk, &options)?;
    if let Some(mesh) = mesh {
      meshes.push((mesh, chunk.position().into()));
    }
  }

  let baker_elapsed = now.elapsed().as_micros();
  let now = Instant::now();

  export(meshes, &format!("{}_png", name));

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
  let _ = read("heightmap");

  Ok(())
}
