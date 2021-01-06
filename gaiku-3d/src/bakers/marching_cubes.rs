use std::collections::HashMap;

use gaiku_common::{glam::Vec3, mint::Vector3, Baker, Chunk, Chunkify, Mesh};

mod tables;

use self::tables::{EDGE_TABLE, TRIANGLE_TABLE};

struct GridCell {
  pub value: [u8; 8],
  pub point: [Vector3<f32>; 8],
}

impl GridCell {
  fn lerp(&self, index1: usize, index2: usize, isolevel: u8) -> Vector3<f32> {
    let mut index1 = index1;
    let mut index2 = index2;

    if self.point[index1] < self.point[index2] {
      std::mem::swap(&mut index1, &mut index2);
    }

    let isolevel = isolevel as f32 / 255.0;

    let point1: Vec3 = self.point[index1].into();
    let point2: Vec3 = self.point[index2].into();
    let iso: Vec3 = [isolevel, isolevel, isolevel].into();

    if (point1 - point2).abs() > [0.00001, 0.00001, 0.00001].into() {
      #[allow(clippy::eq_op)]
      (point1 + (point2 - point1) / (point2 - point1) * (iso - point1)).into()
    } else {
      self.point[index1]
    }
  }
}

pub struct MarchingCubesBaker;

impl MarchingCubesBaker {
  fn polygonize(grid: &GridCell, isolevel: u8, triangles: &mut Vec<[Vector3<f32>; 3]>) {
    let mut cube_index = 0;
    let mut vertex_list = [[0.0, 0.0, 0.0].into(); 12];

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
  fn bake(chunk: &Chunk) -> Option<Mesh> {
    let mut vertices_cache = HashMap::new();
    let mut indices = vec![];

    // TODO: Solve issue where data of next chunk is needed to bake the chunk
    for x in 0..chunk.width() - 1 {
      let fx = x as f32;
      for y in 0..chunk.height() - 1 {
        let fy = y as f32;
        for z in 0..chunk.depth() - 1 {
          let fz = z as f32;

          let grid = GridCell {
            value: [
              chunk.get(x, y, z),
              chunk.get(x + 1, y, z),
              chunk.get(x + 1, y + 1, z),
              chunk.get(x, y + 1, z),
              chunk.get(x, y, z + 1),
              chunk.get(x + 1, y, z + 1),
              chunk.get(x + 1, y + 1, z + 1),
              chunk.get(x, y + 1, z + 1),
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
            for v in &vertex {
              indices.push(Self::index(&mut vertices_cache, *v));
            }
          }
        }
      }
    }

    let mut vertices = vec![[0.0, 0.0, 0.0].into(); vertices_cache.len()];
    for (_, (vertex, index)) in vertices_cache {
      vertices[index as usize] = vertex;
    }

    if !indices.is_empty() {
      Some(Mesh {
        indices,
        vertices,
        normals: vec![],
        colors: vec![],
        uv: vec![],
        tangents: vec![],
      })
    } else {
      None
    }
  }
}
