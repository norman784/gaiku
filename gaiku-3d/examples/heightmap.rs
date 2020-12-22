use gaiku_3d::{bakers::HeightMapBaker, formats::PNGReader};

mod common;

use common::read;

fn main() {
  println!("\nHeightmap");
  println!("---------------------------");
  read::<PNGReader, HeightMapBaker>("heightmap.png", "hm");
}
