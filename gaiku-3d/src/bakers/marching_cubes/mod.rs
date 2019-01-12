use std::collections::HashMap;

use gaiku_common::{
    Baker,
    Chunk,
    Mesh,
    Vec3,
};

mod tables;

use self::tables::{
    EDGE_TABLE,
    TRIANGLE_TABLE,
};

struct GridCell {
    pub value: [f32; 8],
    pub point: [Vec3<f32>; 8],
}

pub struct MarchingCubesBaker;

impl MarchingCubesBaker {
    fn polygonize(grid: &GridCell, isolevel: f32, triangles: &mut Vec<[Vec3<f32>; 3]>) {
        let mut cube_index = 0;
        let mut vertex_list = [Vec3::default(); 12];

        if grid.value[0] < isolevel { cube_index |= 1; }
        if grid.value[1] < isolevel { cube_index |= 2; }
        if grid.value[2] < isolevel { cube_index |= 4; }
        if grid.value[3] < isolevel { cube_index |= 8; }
        if grid.value[4] < isolevel { cube_index |= 16; }
        if grid.value[5] < isolevel { cube_index |= 32; }
        if grid.value[6] < isolevel { cube_index |= 64; }
        if grid.value[7] < isolevel { cube_index |= 128; }

        if EDGE_TABLE[cube_index] == 0 {
            return;
        }

        if (EDGE_TABLE[cube_index] & 1) != 0 {
            vertex_list[0] = linear_interpolation(&grid, 0, 1, isolevel);
        }

        if (EDGE_TABLE[cube_index] & 2) != 0 {
            vertex_list[1] = linear_interpolation(&grid, 1, 2, isolevel);
        }

        if (EDGE_TABLE[cube_index] & 4) != 0 {
            vertex_list[2] = linear_interpolation(&grid, 2, 3, isolevel);
        }

        if (EDGE_TABLE[cube_index] & 8) != 0 {
            vertex_list[3] = linear_interpolation(&grid, 3, 0, isolevel);
        }

        if (EDGE_TABLE[cube_index] & 16) != 0 {
            vertex_list[4] = linear_interpolation(&grid, 4, 5, isolevel);
        }

        if (EDGE_TABLE[cube_index] & 32) != 0 {
            vertex_list[5] = linear_interpolation(&grid, 5, 6, isolevel);
        }

        if (EDGE_TABLE[cube_index] & 64) != 0 {
            vertex_list[6] = linear_interpolation(&grid, 6, 7, isolevel);
        }

        if (EDGE_TABLE[cube_index] & 128) != 0 {
            vertex_list[7] = linear_interpolation(&grid, 7, 4, isolevel);
        }

        if (EDGE_TABLE[cube_index] & 256) != 0 {
            vertex_list[8] = linear_interpolation(&grid, 0, 4, isolevel);
        }

        if (EDGE_TABLE[cube_index] & 512) != 0 {
            vertex_list[9] = linear_interpolation(&grid, 1, 5, isolevel);
        }

        if (EDGE_TABLE[cube_index] & 1024) != 0 {
            vertex_list[10] = linear_interpolation(&grid, 2, 6, isolevel);
        }

        if (EDGE_TABLE[cube_index] & 2048) != 0 {
            vertex_list[11] = linear_interpolation(&grid, 3, 7, isolevel);
        }

        let mut i = 0;

        loop {
            if TRIANGLE_TABLE[cube_index][i] == -1 {
                break;
            }

            triangles.push([
                vertex_list[TRIANGLE_TABLE[cube_index][i    ] as usize],
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
                            chunk.get(x + 0, y + 0, z + 0),
                            chunk.get(x + 1, y + 0, z + 0),
                            chunk.get(x + 1, y + 1, z + 0),
                            chunk.get(x + 0, y + 1, z + 0),
                            chunk.get(x + 0, y + 0, z +  1),
                            chunk.get(x + 1, y + 0, z +  1),
                            chunk.get(x + 1, y +  1, z + 1),
                            chunk.get(x + 0, y +  1, z + 1),
                        ],
                        point: [
                            Vec3{ x: fx + 0.0, y: fy + 0.0, z: fz + 0.0 },
                            Vec3{ x: fx + 1.0, y: fy + 0.0, z: fz + 0.0 },
                            Vec3{ x: fx + 1.0, y: fy + 1.0, z: fz + 0.0 },
                            Vec3{ x: fx + 0.0, y: fy + 1.0, z: fz + 0.0 },
                            Vec3{ x: fx + 0.0, y: fy + 0.0, z: fz + 1.0 },
                            Vec3{ x: fx + 1.0, y: fy + 0.0, z: fz + 1.0 },
                            Vec3{ x: fx + 1.0, y: fy + 1.0, z: fz + 1.0 },
                            Vec3{ x: fx + 0.0, y: fy + 1.0, z: fz + 1.0 },
                        ],
                    };

                    let mut triangles = vec![];
                    Self::polygonize(&grid, 0.001, &mut triangles);

                    for vertex in triangles {
                        for i in 0..3 {
                            indices.push(
                                Self::index(&mut vertices_cache, vertex[i])
                            );
                        }
                    }
                }
            }
        }

        let mut vertices = vec![Vec3::default(); vertices_cache.len()];
        for (vertex, index) in vertices_cache {
            vertices[index] = vertex.clone();
        }

        if indices.len() > 0 {
            Some(
                Mesh {
                    indices,
                    vertices,
                    normals: vec![],
                    colors: vec![],
                    uv: vec![],
                    tangents: vec![],
                }
            )
        } else {
            None
        }
    }
}

fn linear_interpolation(grid: &GridCell, i1: usize, i2: usize, value: f32) -> Vec3<f32> {
    let mut i1 = i1;
    let mut i2 = i2;

    if grid.point[i2] < grid.point[i1] {
        let temp = i1;
        i2 = i1;
        i1 = temp;
    }

    if (grid.value[i1] - grid.value[i2]).abs() > 0.00001 {
        let val1 = Vec3{ x:  grid.value[i1], y: grid.value[i1], z: grid.value[i1] };
        let val2 = Vec3{ x:  grid.value[i2], y: grid.value[i2], z: grid.value[i2] };
        let value = Vec3{ x: value, y: value, z: value };

        grid.point[i1] + (grid.point[i2] - grid.point[i1]) / (val2 - val1) * (value - val1)
    } else {
        grid.point[i1]
    }
}
