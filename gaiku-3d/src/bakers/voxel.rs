use gaiku_common::{prelude::*, Result};

pub struct VoxelBaker;

// TODO: Optimize, don't create faces between chunks if there's a non empty voxel
impl Baker for VoxelBaker {
  type Value = (u8, u8);

  fn bake<C, T, M>(chunk: &C, options: &BakerOptions<T>) -> Result<Option<M>>
  where
    C: Chunkify<Self::Value> + Sizable,
    T: Texturify2d,
    M: Meshify,
  {
    let mut builder = MeshBuilder::create(
      [
        chunk.width() as f32 / 2.0,
        chunk.height() as f32 / 2.0,
        chunk.depth() as f32 / 2.0,
      ],
      [
        chunk.width() as f32,
        chunk.height() as f32,
        chunk.depth() as f32,
      ],
    );

    let x_limit = chunk.width() as usize - 1;
    let y_limit = chunk.height() as usize - 1;
    let z_limit = chunk.depth() as usize - 1;

    for x in 0..chunk.width() as usize {
      for y in 0..chunk.height() as usize {
        for z in 0..chunk.depth() as usize {
          if chunk.is_air(x, y, z) {
            continue;
          }

          let (atlas_index, _) = chunk.get(x, y, z);
          let uv = if let Some(texture) = &options.texture {
            Some(texture.get_uv(atlas_index))
          } else {
            None
          };

          let (fx, fy, fz) = (x as f32, y as f32, z as f32);

          let top_left_back = [fx, fy + 1.0, fz];
          let top_right_back = [fx + 1.0, fy + 1.0, fz];
          let top_right_front = [fx + 1.0, fy + 1.0, fz + 1.0];
          let top_left_front = [fx, fy + 1.0, fz + 1.0];
          let bottom_left_back = [fx, fy, fz];
          let bottom_right_back = [fx + 1.0, fy, fz];
          let bottom_right_front = [fx + 1.0, fy, fz + 1.0];
          let bottom_left_front = [fx, fy, fz + 1.0];

          // Top
          if y == y_limit || chunk.is_air(x, y + 1, z) {
            builder.add_face(
              [
                top_left_back,
                top_right_back,
                top_right_front,
                top_left_front,
              ],
              Some([0.0, 1.0, 0.0]),
              if let Some(uv) = uv {
                Some([uv.0, uv.1, uv.2, uv.3])
              } else {
                None
              },
              atlas_index as u16,
            );
          }

          // Bottom
          if y == 0 || (y > 0 && chunk.is_air(x, y - 1, z)) {
            builder.add_face(
              [
                bottom_right_back,
                bottom_left_back,
                bottom_left_front,
                bottom_right_front,
              ],
              Some([0.0, -1.0, 0.0]),
              if let Some(uv) = uv {
                Some([uv.0, uv.1, uv.2, uv.3])
              } else {
                None
              },
              atlas_index as u16,
            );
          }

          // Left
          if x == 0 || (x > 0 && chunk.is_air(x - 1, y, z)) {
            builder.add_face(
              [
                top_left_back,
                top_left_front,
                bottom_left_front,
                bottom_left_back,
              ],
              Some([-1.0, 0.0, 0.0]),
              if let Some(uv) = uv {
                Some([uv.0, uv.1, uv.2, uv.3])
              } else {
                None
              },
              atlas_index as u16,
            );
          }

          // Right
          if x == x_limit || chunk.is_air(x + 1, y, z) {
            builder.add_face(
              [
                top_right_front,
                top_right_back,
                bottom_right_back,
                bottom_right_front,
              ],
              Some([1.0, 0.0, 0.0]),
              if let Some(uv) = uv {
                Some([uv.0, uv.1, uv.2, uv.3])
              } else {
                None
              },
              atlas_index as u16,
            );
          }

          // Front
          if z == z_limit || chunk.is_air(x, y, z + 1) {
            builder.add_face(
              [
                top_left_front,
                top_right_front,
                bottom_right_front,
                bottom_left_front,
              ],
              Some([0.0, 0.0, 1.0]),
              if let Some(uv) = uv {
                Some([uv.0, uv.1, uv.2, uv.3])
              } else {
                None
              },
              atlas_index as u16,
            );
          }

          // Back
          if z == 0 || chunk.is_air(x, y, z - 1) {
            builder.add_face(
              [
                top_right_back,
                top_left_back,
                bottom_left_back,
                bottom_right_back,
              ],
              Some([0.0, 0.0, -1.0]),
              if let Some(uv) = uv {
                Some([uv.0, uv.1, uv.2, uv.3])
              } else {
                None
              },
              atlas_index as u16,
            );
          }
        }
      }
    }

    Ok(builder.build::<M>())
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use gaiku_common::{chunk::Chunk, mesh::Mesh, texture::Texture2d};

  #[test]
  fn simple_test() {
    let options = Default::default();
    let mut chunk = Chunk::new([0.0, 0.0, 0.0], 1, 1, 1);

    chunk.set(0, 0, 0, (0, 1));

    let mesh = VoxelBaker::bake::<Chunk, Texture2d, Mesh>(&chunk, &options)
      .unwrap()
      .unwrap();

    let positions_count = mesh.get_positions().len();
    let indices_count = mesh.get_indices().len();

    assert_eq!(indices_count, 36);
    assert_eq!(positions_count, 24);
  }
}
