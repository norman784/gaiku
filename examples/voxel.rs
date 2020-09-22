use std::time::Instant;

use gaiku::{
  bakers::Voxel,
  Gaiku,
  formats::Gox,
};

mod common;

use crate::common::export;

fn read(name: &str) -> std::io::Result<()> {
  let now = Instant::now();
  let file = format!(
    "{}/examples/assets/{}.gox",
    env!("CARGO_MANIFEST_DIR"),
    name
  );
  let gaiku = Gaiku::load::<Gox>(&file, [1., 1., 1.]).unwrap();

  let reader_elapsed = now.elapsed().as_secs();
  let now = Instant::now();
  let meshes = gaiku.bake_all::<Voxel>();
  let baker_elapsed = now.elapsed().as_secs();
  let now = Instant::now();
  let mesh_count = meshes.len();

  export(meshes, &format!("{}_vx", name));

  println!(
    "<<{}>> Chunks: {} Reader: {} Baker: {} secs Export: {} secs",
    name,
    mesh_count,
    reader_elapsed,
    baker_elapsed,
    now.elapsed().as_secs()
  );

  Ok(())
}

fn main() -> std::io::Result<()> {
  let _ = read("small_tree");
  let _ = read("terrain");
  let _ = read("planet");

  Ok(())
}
