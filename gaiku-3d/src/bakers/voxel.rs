use gaiku_common::{
    Baker,
    Chunk,
    Mesh,
    Vector3,
};

pub struct VoxelBaker;

impl VoxelBaker {
    fn index(vertices: &mut Vec<Vector3>, vertex: Vector3) -> usize {
        match vertices.iter().position(
            |item| item.x == vertex.x && item.y == vertex.y && item.z == vertex.z
        ) {
            Some(index) => index,
            None => {
                let index = vertices.len();
                vertices.push(vertex);
                index
            }
        }
    }
}

impl Baker for VoxelBaker {
    fn bake(chunk: &Chunk) -> Option<Mesh> {
        let mut indices= vec![];
        let mut vertices = vec![];
        let mut colors =  vec![];
        let xlimit = chunk.width() - 1;
        let ylimit = chunk.height() - 1;
        let zlimit = chunk.depth() - 1;

        for x in 0..chunk.width() {
            let fx = x as f32;
            for y in 0..chunk.height() {
                let fy = y as f32;
                for z in 0..chunk.depth() {
                    let fz = z as f32;

                    if chunk.is_air(x, y, z) {
                        continue;
                    }

                    let top_left_back = Self::index(&mut vertices,[fx - 0.5, fy + 0.5, fz - 0.5].into());
                    let top_right_back = Self::index(&mut vertices,[fx + 0.5, fy + 0.5, fz - 0.5].into());
                    let top_right_front= Self::index(&mut vertices,[fx + 0.5, fy + 0.5, fz + 0.5].into());
                    let top_left_front = Self::index(&mut vertices,[fx - 0.5, fy + 0.5, fz + 0.5].into());

                    let bottom_left_back = Self::index(&mut vertices,[fx - 0.5, fy - 0.5, fz - 0.5].into());
                    let bottom_right_back = Self::index(&mut vertices,[fx + 0.5, fy - 0.5, fz - 0.5].into());
                    let bottom_right_front= Self::index(&mut vertices,[fx + 0.5, fy - 0.5, fz + 0.5].into());
                    let bottom_left_front = Self::index(&mut vertices,[fx - 0.5, fy - 0.5, fz + 0.5].into());

                    // Top
                    if y == ylimit || chunk.is_air(x, y+1, z) {
                        indices.push(top_left_back);
                        indices.push(top_right_back);
                        indices.push(top_left_front);

                        indices.push(top_right_back);
                        indices.push(top_right_front);
                        indices.push(top_left_front);

                        colors.push([0.23, 0.42, 0.12, 1.0].into());
                    }

                    // Bottom
                    if y == 0 || (y > 0 && chunk.is_air(x, y-1, z)) {
                        indices.push(bottom_left_back);
                        indices.push(bottom_right_back);
                        indices.push(bottom_left_front);

                        indices.push(bottom_right_back);
                        indices.push(bottom_right_front);
                        indices.push(bottom_left_front);

                        colors.push([0.24, 0.16, 0.11, 1.0].into());
                    }

                    // Left
                    if x == 0 || (x > 0 && chunk.is_air(x-1, y, z)) {
                        indices.push(top_left_back);
                        indices.push(top_left_front);
                        indices.push(bottom_left_back);

                        indices.push(top_left_front);
                        indices.push(bottom_left_front);
                        indices.push(bottom_left_back);

                        colors.push([0.33, 0.23, 0.16, 1.0].into());
                    }

                    // Right
                    if x == xlimit || chunk.is_air(x+1, y, z) {
                        indices.push(top_right_front);
                        indices.push(top_right_back);
                        indices.push(bottom_right_front);

                        indices.push(top_right_back);
                        indices.push(bottom_right_back);
                        indices.push(bottom_right_front);

                        colors.push([0.33, 0.23, 0.16, 1.0].into());
                    }

                    // Front
                    if z == zlimit || chunk.is_air(x, y, z+1) {
                        indices.push(top_left_front);
                        indices.push(top_right_front);
                        indices.push(bottom_left_front);

                        indices.push(top_right_front);
                        indices.push(bottom_right_front);
                        indices.push(bottom_left_front);

                        colors.push([0.41, 0.29, 0.20, 1.0].into());
                    }

                    // Back
                    if z == 0 || chunk.is_air(x, y, z-1) {
                        indices.push(top_right_back);
                        indices.push(top_left_back);
                        indices.push(bottom_right_back);

                        indices.push(top_left_back);
                        indices.push(bottom_left_back);
                        indices.push(bottom_right_back);

                        colors.push([0.41, 0.29, 0.20, 1.0].into());
                    }
                }
            }
        }

        if indices.len() > 0 {
            Some(
                Mesh {
                    indices,
                    vertices,
                    normals: vec![],
                    colors,
                    uv: vec![],
                    tangents: vec![],
                }
            )
        } else {
            None
        }
    }
}