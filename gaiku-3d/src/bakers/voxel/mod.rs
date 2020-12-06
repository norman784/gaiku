use std::collections::HashMap;

use gaiku_common::{
    mint::{Vector3, Vector4},
    Baker, Chunk, Mesh,
};

pub struct VoxelBaker;

// Each vertex has the following data
struct VertexData {
    position: Vector3<usize>,
    normal: Vector3<i8>,
    color: Vector4<u8>,
    index: u16,
}

impl VertexData {
    pub fn is_same_normal(&self, norm: Vector3<i8>) -> bool {
        norm.x == self.normal.x && norm.y == self.normal.y && norm.z == self.normal.z
    }

    pub fn is_same_color(&self, color: Vector4<u8>) -> bool {
        color.x == self.color.x
            && color.y == self.color.y
            && color.z == self.color.z
            && color.w == self.color.w
    }
}

// TODO: Optimize, don't create faces between chunks if there's a non empty voxel
impl Baker for VoxelBaker {
    fn bake(chunk: &Chunk) -> Option<Mesh> {
        let mut indices = vec![];
        // Hash map in x, y, z coordinates to a list of verts at that coordinates
        let mut vertices: HashMap<(usize, usize, usize), Vec<VertexData>> = HashMap::new();

        let x_limit = chunk.width() - 1;
        let y_limit = chunk.height() - 1;
        let z_limit = chunk.depth() - 1;

        for x in 0..*chunk.width() {
            for y in 0..*chunk.height() {
                for z in 0..*chunk.depth() {
                    if chunk.is_air(x, y, z) {
                        continue;
                    }

                    let color = if let Some(color) = chunk.get_color(x, y, z) {
                        color
                    } else {
                        Vector4 {
                            x: 1,
                            y: 1,
                            z: 1,
                            w: 1,
                        }
                    };

                    let top_left_back = (x, y + 1, z);
                    let top_right_back = (x + 1, y + 1, z);
                    let top_right_front = (x + 1, y + 1, z + 1);
                    let top_left_front = (x, y + 1, z + 1);
                    let bottom_left_back = (x, y, z);
                    let bottom_right_back = (x + 1, y, z);
                    let bottom_right_front = (x + 1, y, z + 1);
                    let bottom_left_front = (x, y, z + 1);

                    // Top
                    if y == y_limit || chunk.is_air(x, y + 1, z) {
                        // indices.push(top_left_back);
                        // indices.push(top_right_back);
                        // indices.push(top_left_front);

                        // indices.push(top_right_back);
                        // indices.push(top_right_front);
                        // indices.push(top_left_front);
                        create_face(
                            &mut indices,
                            &mut vertices,
                            top_left_back,
                            top_right_back,
                            top_right_front,
                            top_left_front,
                            Vector3 { x: 0, y: 1, z: 0 },
                            color,
                        );
                    }

                    // Bottom
                    if y == 0 || (y > 0 && chunk.is_air(x, y - 1, z)) {
                        create_face(
                            &mut indices,
                            &mut vertices,
                            bottom_right_back,
                            bottom_left_back,
                            bottom_left_front,
                            bottom_right_front,
                            Vector3 { x: 0, y: -1, z: 0 },
                            color,
                        );
                    }

                    // Left
                    if x == 0 || (x > 0 && chunk.is_air(x - 1, y, z)) {
                        create_face(
                            &mut indices,
                            &mut vertices,
                            top_left_back,
                            top_left_front,
                            bottom_left_front,
                            bottom_left_back,
                            Vector3 { x: -1, y: 0, z: 0 },
                            color,
                        );
                    }

                    // Right
                    if x == x_limit || chunk.is_air(x + 1, y, z) {
                        create_face(
                            &mut indices,
                            &mut vertices,
                            top_right_front,
                            top_right_back,
                            bottom_right_back,
                            bottom_right_front,
                            Vector3 { x: 1, y: 0, z: 0 },
                            color,
                        );
                    }

                    // Front
                    if z == z_limit || chunk.is_air(x, y, z + 1) {
                        create_face(
                            &mut indices,
                            &mut vertices,
                            top_left_front,
                            top_right_front,
                            bottom_right_front,
                            bottom_left_front,
                            Vector3 { x: 0, y: 0, z: 1 },
                            color,
                        );
                    }

                    // Back
                    if z == 0 || chunk.is_air(x, y, z - 1) {
                        create_face(
                            &mut indices,
                            &mut vertices,
                            top_right_back,
                            top_left_back,
                            bottom_left_back,
                            bottom_right_back,
                            Vector3 { x: 0, y: 0, z: -1 },
                            color,
                        );
                    }
                }
            }
        }

        let mut all_verts: Vec<&VertexData> = vertices.values().flatten().collect();
        all_verts.sort_by_key(|k| k.index);
        let vertices: Vec<Vector3<f32>> = all_verts
            .iter()
            .map(|v| Vector3 {
                x: v.position.x as f32 - 0.5,
                y: v.position.y as f32 - 0.5,
                z: v.position.z as f32 - 0.5,
            })
            .collect();
        let normals: Vec<Vector3<f32>> = all_verts
            .iter()
            .map(|v| {
                let (x, y, z) = (v.normal.x, v.normal.y, v.normal.z);
                let len = ((x.pow(2) + y.pow(2) + z.pow(2)) as f32).sqrt();
                Vector3 {
                    x: x as f32 / len,
                    y: y as f32 / len,
                    z: z as f32 / len,
                }
            })
            .collect();
        let colors: Vec<Vector4<u8>> = all_verts.iter().map(|v| v.color).collect();

        if indices.len() > 0 {
            Some(Mesh {
                indices,
                vertices,
                normals,
                colors,
                uv: vec![],
                tangents: vec![],
            })
        } else {
            None
        }
    }
}

fn get_or_insert<'a>(
    cache: &'a mut HashMap<(usize, usize, usize), Vec<VertexData>>,
    position: (usize, usize, usize),
    color: Vector4<u8>,
    normal: Vector3<i8>,
) -> &'a VertexData {
    let verts = &mut cache.entry(position).or_insert(vec![]);
    for i in 0..verts.len() {
        let vert = &verts[i];
        if vert.is_same_normal(normal) && vert.is_same_color(color) {
            return &cache.get(&position).unwrap()[i];
        }
    }
    let next_index = cache.values().fold(0, |acc, v| acc + v.len());
    let new_vert = VertexData {
        position: Vector3 {
            x: position.0,
            y: position.1,
            z: position.2,
        },
        normal,
        color,
        index: next_index as u16,
    };
    let verts = &mut cache.entry(position).or_insert(vec![]);
    verts.push(new_vert);
    return &cache.get(&position).unwrap().last().unwrap();
}

fn create_face(
    indices: &mut Vec<u16>,
    cache: &mut HashMap<(usize, usize, usize), Vec<VertexData>>,
    p1: (usize, usize, usize),
    p2: (usize, usize, usize),
    p3: (usize, usize, usize),
    p4: (usize, usize, usize),
    normal: Vector3<i8>,
    color: Vector4<u8>,
) {
    [p1, p4, p2, p2, p4, p3].iter().for_each(|p| {
        let v = get_or_insert(cache, *p, color, normal);
        indices.push(v.index);
    });
}
