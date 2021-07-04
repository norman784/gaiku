use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use gaiku::{
  common::{
    chunk::Chunk,
    mesh::Mesh,
    prelude::*,
    texture::{Texture2d, TextureAtlas2d},
    Result,
  },
  GoxReader, VoxelBaker,
};

fn get_chunks(name: &str) -> Result<(Vec<Chunk>, Option<TextureAtlas2d<Texture2d>>)> {
  let file = format!(
    "{}/examples/assets/{}.gox",
    env!("CARGO_MANIFEST_DIR"),
    name
  );

  GoxReader::read(&file)
}

fn voxel_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("Builder");
  let (chunks, texture) = get_chunks("terrain").unwrap();
  let options = BakerOptions {
    texture,
    ..Default::default()
  };
  group.sampling_mode(SamplingMode::Flat);
  group.sample_size(10);

  group.bench_function("NoTree", |b| {
    b.iter(|| {
      let mut meshes: Vec<(Mesh, [f32; 3])> = vec![];

      for chunk in chunks.iter() {
        let mesh = VoxelBaker::bake_with_builder::<Chunk, Texture2d, Mesh, NoTreeBuilder>(
          chunk,
          &options,
          Default::default(),
        )
        .unwrap();
        if let Some(mesh) = mesh {
          meshes.push((mesh, chunk.position()));
        }
      }
    })
  });

  let (chunks, texture) = get_chunks("terrain").unwrap();
  let options = BakerOptions {
    texture,
    ..Default::default()
  };

  group.bench_function("OctTree", |b| {
    b.iter(|| {
      let mut meshes: Vec<(Mesh, [f32; 3])> = vec![];

      for chunk in chunks.iter() {
        let mesh = VoxelBaker::bake_with_builder::<Chunk, Texture2d, Mesh, OctMeshBuilder>(
          chunk,
          &options,
          Default::default(),
        )
        .unwrap();
        if let Some(mesh) = mesh {
          meshes.push((mesh, chunk.position()));
        }
      }
    })
  });

  let (chunks, texture) = get_chunks("terrain").unwrap();
  let options = BakerOptions {
    texture,
    ..Default::default()
  };

  group.bench_function("Rstar", |b| {
    b.iter(|| {
      let mut meshes: Vec<(Mesh, [f32; 3])> = vec![];

      for chunk in chunks.iter() {
        let mesh = VoxelBaker::bake_with_builder::<Chunk, Texture2d, Mesh, RstarMeshBuilder>(
          chunk,
          &options,
          Default::default(),
        )
        .unwrap();
        if let Some(mesh) = mesh {
          meshes.push((mesh, chunk.position()));
        }
      }
    })
  });

  group.finish();
}

criterion_group!(benches, voxel_benchmark);

criterion_main! {
    benches,
}
