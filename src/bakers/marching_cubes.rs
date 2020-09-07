use gaiku_common::{Baker, Chunk, Mesh, MeshBuilder, Vector3};

mod tables;

use self::tables::{EDGE_TABLE, TRIANGLE_TABLE};

struct GridCell {
  pub value: [f32; 8],
  pub point: [Vector3; 8],
}

impl GridCell {
  fn lerp(&self, index1: usize, index2: usize, isolevel: f32) -> Vector3 {
    let mut index1 = index1;
    let mut index2 = index2;

    let p1 = self.point[index1];
    let p2 = self.point[index2];

    if p1[0] < p2[0] && p1[1] < p2[1] && p1[2] < p2[2] {
      let temp = index1;
      index1 = index2;
      index2 = temp;
    }

    let p1 = self.point[index1];
    let p2 = self.point[index2];
    let iso = [isolevel, isolevel, isolevel];

    let abs = [
      (p1[0] - p2[0]).abs(),
      (p1[1] - p2[1]).abs(),
      (p1[2] - p2[2]).abs(),
    ];

    if abs[0] > 0.00001 && abs[1] > 0.0001 && abs[2] > 0.00001 {
      [
        p1[0] + (p2[0] - p1[0]) / (p2[0] - p1[0]) * (iso[0] - p1[0]),
        p1[1] + (p2[1] - p1[1]) / (p2[1] - p1[1]) * (iso[1] - p1[1]),
        p1[2] + (p2[2] - p1[2]) / (p2[2] - p1[2]) * (iso[2] - p1[2]),
      ]
    } else {
      self.point[index1]
    }
  }
}

pub struct MarchingCubesBaker;

impl MarchingCubesBaker {
  fn polygonize(grid: &GridCell, isolevel: f32, builder: &mut MeshBuilder) {
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

      builder.add_triangle_with_color(
        [
          vertex_list[TRIANGLE_TABLE[cube_index][i] as usize],
          vertex_list[TRIANGLE_TABLE[cube_index][i + 1] as usize],
          vertex_list[TRIANGLE_TABLE[cube_index][i + 2] as usize],
        ],
        [50, 200, 50, 255],
      );

      i += 3;
    }
  }
}

impl Baker for MarchingCubesBaker {
  fn bake(chunk: &Chunk) -> Option<Mesh> {
    let mut builder = MeshBuilder::new();
    // TODO: Solve issue where data of next chunk is needed to bake the chunk
    for x in 0..chunk.width() - 1 {
      let fx = x as f32;
      for y in 0..chunk.height() - 1 {
        let fy = y as f32;
        for z in 0..chunk.depth() - 1 {
          let fz = z as f32;

          let grid = GridCell {
            value: [
              chunk.get(x + 0, y + 0, z + 0) as f32 / 255.0,
              chunk.get(x + 1, y + 0, z + 0) as f32 / 255.0,
              chunk.get(x + 1, y + 1, z + 0) as f32 / 255.0,
              chunk.get(x + 0, y + 1, z + 0) as f32 / 255.0,
              chunk.get(x + 0, y + 0, z + 1) as f32 / 255.0,
              chunk.get(x + 1, y + 0, z + 1) as f32 / 255.0,
              chunk.get(x + 1, y + 1, z + 1) as f32 / 255.0,
              chunk.get(x + 0, y + 1, z + 1) as f32 / 255.0,
            ],
            point: [
              [fx + 0.0, fy + 0.0, fz + 0.0],
              [fx + 1.0, fy + 0.0, fz + 0.0],
              [fx + 1.0, fy + 1.0, fz + 0.0],
              [fx + 0.0, fy + 1.0, fz + 0.0],
              [fx + 0.0, fy + 0.0, fz + 1.0],
              [fx + 1.0, fy + 0.0, fz + 1.0],
              [fx + 1.0, fy + 1.0, fz + 1.0],
              [fx + 0.0, fy + 1.0, fz + 1.0],
            ],
          };

          Self::polygonize(&grid, 0.001, &mut builder);
        }
      }
    }

    builder.build()
  }
}
