use std::time::Instant;

use gaiku_3d::common::{Baker, Chunk, FileFormat};

mod exporter;

pub use self::exporter::export;

pub fn bake<T>(name: &str, chunks: Vec<Chunk>, now: Instant, suffix: &str)
where
  T: Baker,
{
  let mut meshes = vec![];

  let reader_elapsed = now.elapsed().as_micros();
  let now = Instant::now();

  for chunk in chunks.iter() {
    let mesh = T::bake(chunk);
    if let Some(mesh) = mesh {
      meshes.push((mesh, chunk.position()));
    }
  }

  let baker_elapsed = now.elapsed().as_micros();

  export(meshes, &format!("{}_{}", name, suffix));

  println!(
    "\t<<{}>>\n\t\tChunk count: {}\n\t\tReader: {} micros\n\t\tBaker: {} micros",
    name,
    chunks.len(),
    reader_elapsed,
    baker_elapsed,
  );
}

pub fn read<F, B>(name: &str, suffix: &str)
where
  F: FileFormat,
  B: Baker,
{
  let file = format!("{}/examples/assets/{}", env!("CARGO_MANIFEST_DIR"), name);
  bake::<B>(name, F::read(&file), Instant::now(), suffix);
}
