use crate::common::{Baker, Chunk, Mesh, MeshBuilder};

pub struct HeightMapBaker;

impl Baker for HeightMapBaker {
  fn bake(chunk: &Chunk) -> Option<Mesh> {
    let mut builder = MeshBuilder::new();
    let height = 30.0;

    // FIXME: Some vertices are wrong, cross the entire mesh
    for x in 0..chunk.width() - 1 {
      let fx = x as f32;
      for y in 0..chunk.height() - 1 {
        if chunk.is_empty(x, y, 0) {
          continue;
        }

        let fz = y as f32;
        let lb = (chunk.get(x, y, 0) as f32 * height) / 255.0;
        let lf = (chunk.get(x, y + 1, 0) as f32 * height) / 255.0;
        let rb = (chunk.get(x + 1, y, 0) as f32 * height) / 255.0;
        let rf = (chunk.get(x + 1, y + 1, 0) as f32 * height) / 255.0;

        let left_back = [fx - 0.5, lb, fz - 0.5];
        let right_back = [fx + 0.5, rb, fz - 0.5];
        let right_front = [fx + 0.5, rf, fz + 0.5];
        let left_front = [fx - 0.5, lf, fz + 0.5];

        builder.add_triangle_with_color([left_back, left_front, right_back], [50, 200, 50, 255]);
        builder.add_triangle_with_color([right_back, left_front, right_front], [50, 200, 50, 255]);
      }
    }

    builder.build()
  }
}
