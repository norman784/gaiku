use criterion::{criterion_group, criterion_main, Criterion};
use gaiku::{
  common::{
    chunk::Chunk,
    mesh::Mesh,
    prelude::*,
    texture::{Texture2d, TextureAtlas2d},
    Result,
  },
  GoxReader, MarchingCubesBaker,
};

fn get_chunks(name: &str) -> Result<(Vec<Chunk>, Option<TextureAtlas2d<Texture2d>>)> {
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
      let mut meshes: Vec<(Mesh, [f32; 3])> = vec![];

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
      let mut meshes: Vec<(Mesh, [f32; 3])> = vec![];

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
      let mut meshes: Vec<(Mesh, [f32; 3])> = vec![];

      for chunk in chunks.iter() {
        let mesh = MarchingCubesBaker::bake(chunk, &options).unwrap();
        if let Some(mesh) = mesh {
          meshes.push((mesh, chunk.position()));
        }
      }
    })
  });

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
  group.bench_function("Small Checkerboard", |b| {
    b.iter(|| {
      MarchingCubesBaker::bake::<Chunk, Texture2d, Mesh>(&chunk, &options).unwrap();
    })
  });

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
  group.bench_function("Medium Checkerboard", |b| {
    b.iter(|| {
      MarchingCubesBaker::bake::<Chunk, Texture2d, Mesh>(&chunk, &options).unwrap();
    })
  });

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
  group.bench_function("Large Checkerboard", |b| {
    b.iter(|| {
      MarchingCubesBaker::bake::<Chunk, Texture2d, Mesh>(&chunk, &options).unwrap();
    })
  });

  group.finish();
}

criterion_group!(benches, marching_cubes_benchmark);

criterion_main! {
    benches,
}
