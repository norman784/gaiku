use gaiku_3d::{bakers::MarchingCubesBaker, formats::GoxReader};

mod common;

use common::read;

fn main() {
  println!("\nMarching cubes");
  println!("---------------------------");
  read::<GoxReader, MarchingCubesBaker>("small_tree.gox", "mc");
  read::<GoxReader, MarchingCubesBaker>("terrain.gox", "mc");
  read::<GoxReader, MarchingCubesBaker>("planet.gox", "mc");
}
