use gaiku_common::{mint::Vector3, prelude::*, Result};
use glam::Vec3;

mod tables;

use self::tables::{EDGE_TABLE, TRIANGLE_TABLE};

#[derive(Debug)]
struct GridCell {
  pub value: [u8; 8],
  pub point: [Vector3<f32>; 8],
}

impl GridCell {
  fn lerp(&self, index1: usize, index2: usize, isolevel: u8) -> [f32; 3] {
    let mut index1 = index1;
    let mut index2 = index2;

    if self.point[index2] < self.point[index1] {
      std::mem::swap(&mut index1, &mut index2);
    }

    let isolevel = isolevel as f32 / 255.0;

    let point1: Vec3 = self.point[index1].into();
    let point2: Vec3 = self.point[index2].into();

    if (point1 - point2).abs() > [0.0001, 0.0001, 0.0001].into() {
      let value1: Vec3 = [
        self.value[index1] as f32 / 255.0,
        self.value[index1] as f32 / 255.0,
        self.value[index1] as f32 / 255.0,
      ]
      .into();
      let value2: Vec3 = [
        self.value[index2] as f32 / 255.0,
        self.value[index2] as f32 / 255.0,
        self.value[index2] as f32 / 255.0,
      ]
      .into();
      let value: Vec3 = [isolevel, isolevel, isolevel].into();

      (point1 + (point2 - point1) / (value2 - value1) * (value - value1)).into()
    } else {
      self.point[index1].into()
    }
  }
}

/// Implementation of the marching cubes terrain generation.
pub struct MarchingCubesBaker;

impl MarchingCubesBaker {
  fn polygonize(grid: &GridCell, isolevel: u8, triangles: &mut Vec<[[f32; 3]; 3]>) {
    let mut cube_index = 0;
    let mut vertex_list = [[0.0, 0.0, 0.0]; 12];

    if grid.value[0] < isolevel {
      cube_index |= 1;
    }
    if grid.value[1] < isolevel {
      cube_index |= 2;
    }
    if grid.value[2] < isolevel {
      cube_index |= 4;
    }
    if grid.value[3] < isolevel {
      cube_index |= 8;
    }
    if grid.value[4] < isolevel {
      cube_index |= 16;
    }
    if grid.value[5] < isolevel {
      cube_index |= 32;
    }
    if grid.value[6] < isolevel {
      cube_index |= 64;
    }
    if grid.value[7] < isolevel {
      cube_index |= 128;
    }

    if EDGE_TABLE[cube_index] == 0 {
      return;
    }

    if (EDGE_TABLE[cube_index] & 1) != 0 {
      vertex_list[0] = grid.lerp(0, 1, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 2) != 0 {
      vertex_list[1] = grid.lerp(1, 2, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 4) != 0 {
      vertex_list[2] = grid.lerp(2, 3, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 8) != 0 {
      vertex_list[3] = grid.lerp(3, 0, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 16) != 0 {
      vertex_list[4] = grid.lerp(4, 5, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 32) != 0 {
      vertex_list[5] = grid.lerp(5, 6, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 64) != 0 {
      vertex_list[6] = grid.lerp(6, 7, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 128) != 0 {
      vertex_list[7] = grid.lerp(7, 4, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 256) != 0 {
      vertex_list[8] = grid.lerp(0, 4, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 512) != 0 {
      vertex_list[9] = grid.lerp(1, 5, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 1024) != 0 {
      vertex_list[10] = grid.lerp(2, 6, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 2048) != 0 {
      vertex_list[11] = grid.lerp(3, 7, isolevel);
    }

    let mut i = 0;

    loop {
      if TRIANGLE_TABLE[cube_index][i] == -1 {
        break;
      }

      triangles.push([
        vertex_list[TRIANGLE_TABLE[cube_index][i] as usize],
        vertex_list[TRIANGLE_TABLE[cube_index][i + 1] as usize],
        vertex_list[TRIANGLE_TABLE[cube_index][i + 2] as usize],
      ]);

      i += 3;
    }
  }
}

impl Baker for MarchingCubesBaker {
  type Value = (u8, u8);

  fn bake<C, T, M>(chunk: &C, _options: &BakerOptions<T>) -> Result<Option<M>>
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

    // TODO: Solve issue where data of next chunk is needed to bake the chunk
    for x in 0..chunk.width() as usize - 1 {
      let fx = x as f32;
      for y in 0..chunk.height() as usize - 1 {
        let fy = y as f32;
        for z in 0..chunk.depth() as usize - 1 {
          let fz = z as f32;

          let grid = GridCell {
            value: [
              chunk.get(x, y, z).1,
              chunk.get(x + 1, y, z).1,
              chunk.get(x + 1, y + 1, z).1,
              chunk.get(x, y + 1, z).1,
              chunk.get(x, y, z + 1).1,
              chunk.get(x + 1, y, z + 1).1,
              chunk.get(x + 1, y + 1, z + 1).1,
              chunk.get(x, y + 1, z + 1).1,
            ],
            point: [
              [fx + 0.0, fy + 0.0, fz + 0.0].into(),
              [fx + 1.0, fy + 0.0, fz + 0.0].into(),
              [fx + 1.0, fy + 1.0, fz + 0.0].into(),
              [fx + 0.0, fy + 1.0, fz + 0.0].into(),
              [fx + 0.0, fy + 0.0, fz + 1.0].into(),
              [fx + 1.0, fy + 0.0, fz + 1.0].into(),
              [fx + 1.0, fy + 1.0, fz + 1.0].into(),
              [fx + 0.0, fy + 1.0, fz + 1.0].into(),
            ],
          };

          let mut triangles = vec![];
          Self::polygonize(&grid, 1, &mut triangles);

          for vertex in triangles {
            builder.add_triangle(vertex, None, None, 0);
          }
        }
      }
    }

    Ok(builder.build::<M>())
  }
}
