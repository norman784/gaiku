use std::collections::HashMap;

use crate::common::{Baker, Chunk, Mesh};

pub struct HeightMapBaker;

impl Baker for HeightMapBaker {
    fn bake(chunk: &Chunk) -> Option<Mesh> {
        let mut indices = vec![];
        let mut vertices_cache = HashMap::new();
        let colors = vec![];
        let height = 30;

        for x in 0..chunk.width() - 1 {
            let fx = x as f32;
            for y in 0..chunk.height() - 1 {
                if chunk.is_air(x, y, 0) {
                    continue;
                }
                let fz = y as f32;
                let lb = (chunk.get(x, y, 0) * height) as f32 / 255.0;
                let lf = (chunk.get(x, y + 1, 0) * height) as f32 / 255.0;
                let rb = (chunk.get(x + 1, y, 0) * height) as f32 / 255.0;
                let rf = (chunk.get(x + 1, y + 1, 0) * height) as f32 / 255.0;

                let left_back = Self::index(&mut vertices_cache, [fx - 0.5, lb, fz - 0.5].into());
                let right_back = Self::index(&mut vertices_cache, [fx + 0.5, rb, fz - 0.5].into());
                let right_front = Self::index(&mut vertices_cache, [fx + 0.5, rf, fz + 0.5].into());
                let left_front = Self::index(&mut vertices_cache, [fx - 0.5, lf, fz + 0.5].into());

                indices.push(left_back);
                indices.push(right_back);
                indices.push(left_front);

                indices.push(right_back);
                indices.push(right_front);
                indices.push(left_front);
            }
        }

        let mut vertices = vec![[0.0, 0.0, 0.0].into(); vertices_cache.len()];
        for (_, (vertex, index)) in vertices_cache {
            vertices[index as usize] = vertex.clone();
        }

        if indices.len() > 0 {
            Some(Mesh {
                indices,
                vertices,
                normals: vec![],
                colors,
                uv: vec![],
                tangents: vec![],
            })
        } else {
            None
        }
    }
}
