use criterion::{criterion_group, Criterion};
use gaiku_3d::{
  bakers::MarchingCubesBaker,
  common::{prelude::*, Chunk, Result, TextureAtlas2d},
  formats::GoxReader,
};

fn get_chunks(name: &str) -> Result<(Vec<Chunk>, Option<TextureAtlas2d>)> {
  let file = format!(
    "{}/examples/assets/{}.gox",
    env!("CARGO_MANIFEST_DIR"),
    name
  );

  GoxReader::read(&file)
}

fn marching_cubes_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("Marching cubes");
  let (chunks, texture) = get_chunks("terrain").unwrap();
  let options = BakerOptions {
    texture,
    ..Default::default()
  };

  group.bench_function("Terrain", |b| {
    b.iter(|| {
      let mut meshes = vec![];

      for chunk in chunks.iter() {
        let mesh = MarchingCubesBaker::bake(chunk, &options).unwrap();
        if let Some(mesh) = mesh {
          meshes.push((mesh, chunk.position()));
        }
      }
    })
  });

  let (chunks, texture) = get_chunks("planet").unwrap();
  let options = BakerOptions {
    texture,
    ..Default::default()
  };

  group.bench_function("Planet", |b| {
    b.iter(|| {
      let mut meshes = vec![];

      for chunk in chunks.iter() {
        let mesh = MarchingCubesBaker::bake(chunk, &options).unwrap();
        if let Some(mesh) = mesh {
          meshes.push((mesh, chunk.position()));
        }
      }
    })
  });

  let (chunks, texture) = get_chunks("small_tree").unwrap();
  let options = BakerOptions {
    texture,
    ..Default::default()
  };

  group.bench_function("Small tree", |b| {
    b.iter(|| {
      let mut meshes = vec![];

      for chunk in chunks.iter() {
        let mesh = MarchingCubesBaker::bake(chunk, &options).unwrap();
        if let Some(mesh) = mesh {
          meshes.push((mesh, chunk.position()));
        }
      }
    })
  });

  group.finish();
}

criterion_group!(benches, marching_cubes_benchmark);
