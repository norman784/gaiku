use crate::common::{prelude::*, Chunk, Result};

pub struct HeightMapBaker;

impl Baker for HeightMapBaker {
  fn bake(chunk: &Chunk, _options: &BakerOptions) -> Result<Option<Mesh>> {
    let height = 30;
    let mut builder = MeshBuilder::create();

    println!("{}", chunk.width() * chunk.height());

    for x in 0..chunk.width() - 1 {
      for z in 0..chunk.height() - 1 {
        if chunk.is_air(x, z, 0) {
          continue;
        }

        println!("{} {}", x, y);

        let lb = (chunk.get(x, y, 0).0 as u32 * height) as f32 / 255.0;
        let lf = (chunk.get(x, y + 1, 0).0 as u32 * height) as f32 / 255.0;
        let rb = (chunk.get(x + 1, y, 0).0 as u32 * height) as f32 / 255.0;
        let rf = (chunk.get(x + 1, y + 1, 0).0 as u32 * height) as f32 / 255.0;

        let left_back = [x - 1, lb, z - 1].into();
        let right_back = [x + 1, rb, z - 1].into();
        let right_front = [x + 1, rf, z + 1].into();
        let left_front = [x - 1, lf, z + 1].into();

        builder.add_triangle([left_back, right_back, left_front], None, None, None);
        builder.add_triangle([right_back, right_front, left_front], None, None, None);
      }
    }

    println!("{:?}", &builder);

    Ok(builder.build(-0.5))
  }
}
