use gaiku_common::{prelude::*, Result};
use std::marker::PhantomData;

/// Implementation of a naive heightmap terrain generation.
pub struct HeightMapBaker;

impl HeightMapBaker {
  fn bake_with_builder<C, T, M, MB>(
    chunk: &C,
    _options: &BakerOptions<T>,
    _mark: PhantomData<MB>,
  ) -> Result<Option<M>>
  where
    C: Chunkify<<Self as Baker>::Value> + Atlasify<<Self as Baker>::AtlasValue> + Sizable,
    T: Texturify2d,
    M: Meshify,
    MB: MeshBuilder,
  {
    let height = 30.;
    let mut builder: MB = MB::create(
      [
        chunk.width() as f32 / 2.0,
        height as f32 / 2.0,
        chunk.height() as f32 / 2.0,
      ],
      [chunk.width() as f32, height as f32, chunk.height() as f32],
    );

    for x in 0..chunk.width() as usize - 1 {
      for y in 0..chunk.height() as usize - 1 {
        if chunk.is_air(x, y, 0) {
          continue;
        }

        let fx = x as f32;
        let fz = y as f32;

        let lb = chunk.get(x, y, 0) * height / 255.0;
        let lf = chunk.get(x, y + 1, 0) * height / 255.0;
        let rb = chunk.get(x + 1, y, 0) * height / 255.0;
        let rf = chunk.get(x + 1, y + 1, 0) * height / 255.0;

        let left_back = [fx, lb, fz];
        let right_back = [fx + 1.0, rb, fz];
        let right_front = [fx + 1.0, rf, fz + 1.0];
        let left_front = [fx, lf, fz + 1.0];

        builder.add_triangle([left_front, right_back, left_back], None, None, 0);
        builder.add_triangle([right_front, right_back, left_front], None, None, 0);
      }
    }

    Ok(builder.build::<M>())
  }
}

impl Baker for HeightMapBaker {
  type Value = f32;
  type AtlasValue = u8;

  fn bake<C, T, M>(chunk: &C, options: &BakerOptions<T>) -> Result<Option<M>>
  where
    C: Chunkify<Self::Value> + Atlasify<Self::AtlasValue> + Sizable,
    T: Texturify2d,
    M: Meshify,
  {
    if options.remove_duplicate_verts {
      Self::bake_with_builder::<C, T, M, DefaultMeshBuilder>(chunk, options, Default::default())
    } else {
      Self::bake_with_builder::<C, T, M, NoTreeBuilder>(chunk, options, Default::default())
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use gaiku_common::{chunk::Chunk, mesh::Mesh, texture::Texture2d};
  type BakerType = HeightMapBaker;

  #[test]
  fn simple_test_heightmap() {
    let options = BakerOptions {
      remove_duplicate_verts: true,
      ..Default::default()
    };
    let mut chunk = Chunk::new([0.0, 0.0, 0.0], 3, 3, 3);

    chunk.set(0, 0, 0, 0.);
    chunk.set(0, 1, 0, 0.);
    chunk.set(0, 2, 0, 0.);
    chunk.set(1, 0, 0, 0.);
    chunk.set(1, 1, 0, 1.);
    chunk.set(1, 2, 0, 0.);
    chunk.set(2, 0, 0, 0.);
    chunk.set(2, 1, 0, 0.);
    chunk.set(2, 2, 0, 0.);

    let mesh = BakerType::bake::<Chunk, Texture2d, Mesh>(&chunk, &options)
      .unwrap()
      .unwrap();

    let positions_count = mesh.get_positions().len();
    let indices_count = mesh.get_indices().len();

    assert_eq!(indices_count, 6);
    assert_eq!(positions_count, 4);
  }
}
