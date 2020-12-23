use std::time::Instant;

use gaiku_3d::{
  bakers::voxel::{GreedyMeshingBaker, LinearBaker, PyramidBaker},
  common::{Baker, Chunk, Chunkify},
  formats::GoxReader,
};

mod common;

use common::{bake, read};

fn run<T>(algorithm: &str, suffix: &str)
where
  T: Baker,
{
  println!("\n{}", algorithm);
  println!("---------------------------");
  /*
  read::<GoxReader, T>("small_tree.gox", suffix);
  read::<GoxReader, T>("terrain.gox", suffix);
  read::<GoxReader, T>("planet.gox", suffix);
    */
  // Cube
  let mut chunk = Chunk::new([0.0, 0.0, 0.0], 16, 16, 16);

  for x in 0..chunk.width() {
    for y in 0..chunk.width() {
      for z in 0..chunk.width() {
        chunk.set(x, y, z, 255);
      }
    }
  }

  bake::<T>("cube", vec![chunk], Instant::now(), suffix);

  // Custom chunk
  let mut chunk = Chunk::new([0.0, 0.0, 0.0], 16, 16, 16);

  for x in 0..chunk.width() {
    for y in 0..chunk.width() {
      for z in 0..chunk.width() {
        chunk.set(x, y, z, 255);
      }
    }
  }

  chunk.set(2, 0, 0, 0);
  chunk.set(0, 1, 0, 0);
  chunk.set(3, 1, 0, 0);
  chunk.set(4, 1, 0, 0);
  chunk.set(1, 3, 0, 0);

  bake::<T>("custom chunk", vec![chunk], Instant::now(), suffix);
}

fn main() {
  //run::<LinearBaker>("Linear", "vx_l");
  run::<GreedyMeshingBaker>("Greedy meshing", "vx_gm");
  //run::<PyramidBaker>("Pyramid", "vx_p");
}
