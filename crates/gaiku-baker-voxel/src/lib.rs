use gaiku_common::{Baker, Chunk, Color, Mesh, MeshBuilder, Vector3};

pub struct Voxel;

// TODO: Optimize, don't create faces between chunks if there's a non empty voxel
impl Baker for Voxel {
  fn bake(chunk: &Chunk) -> Option<Mesh> {
    let mut builder = MeshBuilder::new();
    let x_limit = chunk.width() - 1;
    let y_limit = chunk.height() - 1;
    let z_limit = chunk.depth() - 1;

    for x in 0..chunk.width() {
      let fx = x as f32;
      for y in 0..chunk.height() {
        let fy = y as f32;
        for z in 0..chunk.depth() {
          let fz = z as f32;

          if chunk.is_empty(x, y, z) {
            continue;
          }

          let color = if let Some(color) = chunk.get_color(x, y, z) {
            color
          } else {
            [1, 1, 1, 1]
          };

          let top_left_back = [fx - 0.5, fy + 0.5, fz - 0.5];
          let top_right_back = [fx + 0.5, fy + 0.5, fz - 0.5];
          let top_right_front = [fx + 0.5, fy + 0.5, fz + 0.5];
          let top_left_front = [fx - 0.5, fy + 0.5, fz + 0.5];
          let bottom_left_back = [fx - 0.5, fy - 0.5, fz - 0.5];
          let bottom_right_back = [fx + 0.5, fy - 0.5, fz - 0.5];
          let bottom_right_front = [fx + 0.5, fy - 0.5, fz + 0.5];
          let bottom_left_front = [fx - 0.5, fy - 0.5, fz + 0.5];

          // Top
          if y == y_limit || chunk.is_empty(x, y + 1, z) {
            create_face(
              &mut builder,
              top_left_back,
              top_right_back,
              top_right_front,
              top_left_front,
              color,
            );
          }

          // Bottom
          if y == 0 || (y > 0 && chunk.is_empty(x, y - 1, z)) {
            create_face(
              &mut builder,
              bottom_right_back,
              bottom_left_back,
              bottom_left_front,
              bottom_right_front,
              color,
            );
          }

          // Left
          if x == 0 || (x > 0 && chunk.is_empty(x - 1, y, z)) {
            create_face(
              &mut builder,
              top_left_back,
              top_left_front,
              bottom_left_front,
              bottom_left_back,
              color,
            );
          }

          // Right
          if x == x_limit || chunk.is_empty(x + 1, y, z) {
            create_face(
              &mut builder,
              top_right_front,
              top_right_back,
              bottom_right_back,
              bottom_right_front,
              color,
            );
          }

          // Front
          if z == z_limit || chunk.is_empty(x, y, z + 1) {
            create_face(
              &mut builder,
              top_left_front,
              top_right_front,
              bottom_right_front,
              bottom_left_front,
              color,
            );
          }

          // Back
          if z == 0 || chunk.is_empty(x, y, z - 1) {
            create_face(
              &mut builder,
              top_right_back,
              top_left_back,
              bottom_left_back,
              bottom_right_back,
              color,
            );
          }
        }
      }
    }

    builder.build()
  }
}

fn create_face(
  builder: &mut MeshBuilder,
  p1: Vector3,
  p2: Vector3,
  p3: Vector3,
  p4: Vector3,
  color: Color,
) {
  builder.add_triangle_with_color([p1, p4, p2], color);
  builder.add_triangle_with_color([p2, p4, p3], color);
}
