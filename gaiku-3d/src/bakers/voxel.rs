use gaiku_common::{
    Baker,
    Chunk,
    Mesh,
    Vector3,
};

pub struct VoxelBaker;

impl VoxelBaker {
    fn index(vertices: &mut Vec<Vector3>, vertex: Vector3) -> i32 {
        match vertices.iter().position(
            |item| item[0] == vertex[0] && item[1] == vertex[1] && item[2] == vertex[2]
        ) {
            Some(index) => index as i32,
            None => {
                let index = vertices.len() as i32;
                vertices.push(vertex);
                index
            }
        }
    }
}

impl Baker for VoxelBaker {
    fn bake(chunk: &Chunk) -> Mesh {
        let mut indices= vec![];
        let mut vertices = vec![];
        let mut colors =  vec![];
        let limit = (chunk.size() - 1) as i32;

        for ux in 0..chunk.size() {
            for uy in 0..chunk.size() {
                for uz in 0..chunk.size() {
                    let x = ux as i32;
                    let y = uy as i32;
                    let z = uz as i32;

                    if chunk.is_empty((x, y+1, z)) {
                        continue;
                    }

                    let fx = x as f32;
                    let fy = x as f32;
                    let fz = x as f32;

                    let top_left_back = Self::index(&mut vertices,[fx - 0.5, fy + 0.5, fz - 0.5]);
                    let top_right_back = Self::index(&mut vertices,[fx + 0.5, fy + 0.5, fz - 0.5]);
                    let top_right_front= Self::index(&mut vertices,[fx + 0.5, fy + 0.5, fz + 0.5]);
                    let top_left_front = Self::index(&mut vertices,[fx - 0.5, fy, fz + 0.5]);

                    let bottom_left_back = Self::index(&mut vertices,[fx - 0.5, fy - 0.5, fz - 0.5]);
                    let bottom_right_back = Self::index(&mut vertices,[fx + 0.5, fy - 0.5, fz - 0.5]);
                    let bottom_right_front= Self::index(&mut vertices,[fx + 0.5, fy - 0.5, fz + 0.5]);
                    let bottom_left_front = Self::index(&mut vertices,[fx - 0.5, fy - 0.5, fz + 0.5]);

                    // Top
                    if y == 0 || chunk.is_empty((x, y+1, z)) {
                        indices.push(top_left_back);
                        indices.push(top_right_back);
                        indices.push(top_left_front);

                        indices.push(top_right_back);
                        indices.push(top_right_front);
                        indices.push(top_left_front);

                        colors.push([0.23, 0.42, 0.12, 1.0]);
                    }

                    // Bottom
                    if y == limit || chunk.is_empty((x, y-1, z)) {
                        indices.push(bottom_left_back);
                        indices.push(bottom_right_back);
                        indices.push(bottom_left_front);

                        indices.push(bottom_right_back);
                        indices.push(bottom_right_front);
                        indices.push(bottom_left_front);

                        colors.push([0.24, 0.16, 0.11, 1.0]);
                    }

                    // Left
                    if x == 0 || chunk.is_empty((x-1, y, z)) {
                        indices.push(top_left_back);
                        indices.push(top_left_front);
                        indices.push(bottom_left_back);

                        indices.push(top_left_front);
                        indices.push(bottom_left_front);
                        indices.push(bottom_left_back);

                        colors.push([0.33, 0.23, 0.16, 1.0]);
                    }

                    // Right
                    if x == limit || chunk.is_empty((x+1, y, z)) {
                        indices.push(top_right_front);
                        indices.push(top_right_back);
                        indices.push(bottom_right_front);

                        indices.push(top_right_back);
                        indices.push(bottom_right_back);
                        indices.push(bottom_right_front);

                        colors.push([0.33, 0.23, 0.16, 1.0]);
                    }

                    // Front
                    if y == 0 || chunk.is_empty((x, y+1, z)) {
                        indices.push(top_left_front);
                        indices.push(top_right_front);
                        indices.push(bottom_left_front);

                        indices.push(top_right_front);
                        indices.push(bottom_right_front);
                        indices.push(bottom_left_front);

                        colors.push([0.41, 0.29, 0.20, 1.0]);
                    }

                    // Back
                    if y == 0 || chunk.is_empty((x, y+1, z)) {
                        indices.push(top_right_back);
                        indices.push(top_left_back);
                        indices.push(bottom_right_back);

                        indices.push(top_left_back);
                        indices.push(bottom_left_back);
                        indices.push(bottom_right_back);

                        colors.push([0.41, 0.29, 0.20, 1.0]);
                    }
                }
            }
        }

        Mesh {
            indices,
            vertices,
            normals: vec![],
            colors,
            uv: vec![],
            tangents: vec![],
        }
    }
}