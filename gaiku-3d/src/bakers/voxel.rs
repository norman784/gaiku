use std::collections::HashMap;

use gaiku_common::{
  mint::{Vector3, Vector4},
  Baker, Chunk, Chunkify, Mesh,
};

pub struct VoxelBaker;

// TODO: Optimize, don't create faces between chunks if there's a non empty voxel
impl Baker for VoxelBaker {
  fn bake(chunk: &Chunk) -> Option<Mesh> {
    let mut indices = vec![];
    let mut vertices_cache = HashMap::new();
    // FIXME calculate correctly how many indices we need
    let mut colors = vec![
      Vector4 {
        x: 0,
        y: 0,
        z: 0,
        w: 0
      };
      chunk.width() * chunk.height() * chunk.depth()
    ];
    let x_limit = chunk.width() - 1;
    let y_limit = chunk.height() - 1;
    let z_limit = chunk.depth() - 1;

    for x in 0..chunk.width() {
      let fx = x as f32;
      for y in 0..chunk.height() {
        let fy = y as f32;
        for z in 0..chunk.depth() {
          let fz = z as f32;

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

          let top_left_back =
            Self::index(&mut vertices_cache, [fx - 0.5, fy + 0.5, fz - 0.5].into());
          let top_right_back =
            Self::index(&mut vertices_cache, [fx + 0.5, fy + 0.5, fz - 0.5].into());
          let top_right_front =
            Self::index(&mut vertices_cache, [fx + 0.5, fy + 0.5, fz + 0.5].into());
          let top_left_front =
            Self::index(&mut vertices_cache, [fx - 0.5, fy + 0.5, fz + 0.5].into());
          let bottom_left_back =
            Self::index(&mut vertices_cache, [fx - 0.5, fy - 0.5, fz - 0.5].into());
          let bottom_right_back =
            Self::index(&mut vertices_cache, [fx + 0.5, fy - 0.5, fz - 0.5].into());
          let bottom_right_front =
            Self::index(&mut vertices_cache, [fx + 0.5, fy - 0.5, fz + 0.5].into());
          let bottom_left_front =
            Self::index(&mut vertices_cache, [fx - 0.5, fy - 0.5, fz + 0.5].into());

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
              &mut colors,
              top_left_back,
              top_right_back,
              top_right_front,
              top_left_front,
              color,
            );
          }

          // Bottom
          if y == 0 || (y > 0 && chunk.is_air(x, y - 1, z)) {
            create_face(
              &mut indices,
              &mut colors,
              bottom_right_back,
              bottom_left_back,
              bottom_left_front,
              bottom_right_front,
              color,
            );
          }

          // Left
          if x == 0 || (x > 0 && chunk.is_air(x - 1, y, z)) {
            create_face(
              &mut indices,
              &mut colors,
              top_left_back,
              top_left_front,
              bottom_left_front,
              bottom_left_back,
              color,
            );
          }

          // Right
          if x == x_limit || chunk.is_air(x + 1, y, z) {
            create_face(
              &mut indices,
              &mut colors,
              top_right_front,
              top_right_back,
              bottom_right_back,
              bottom_right_front,
              color,
            );
          }

          // Front
          if z == z_limit || chunk.is_air(x, y, z + 1) {
            create_face(
              &mut indices,
              &mut colors,
              top_left_front,
              top_right_front,
              bottom_right_front,
              bottom_left_front,
              color,
            );
          }

          // Back
          if z == 0 || chunk.is_air(x, y, z - 1) {
            create_face(
              &mut indices,
              &mut colors,
              top_right_back,
              top_left_back,
              bottom_left_back,
              bottom_right_back,
              color,
            );
          }
        }
      }
    }

    let mut vertices: Vec<Vector3<f32>> = vec![[0.0, 0.0, 0.0].into(); vertices_cache.len()];
    for (_, (vertex, index)) in vertices_cache {
      vertices[index as usize] = vertex;
    }

    if !indices.is_empty() {
      let end = vertices.len();
      Some(Mesh {
        indices,
        vertices,
        normals: vec![],
        colors: colors[0..end].iter().copied().collect::<Vec<_>>(),
        uv: vec![],
        tangents: vec![],
      })
    } else {
      None
    }
  }
}

fn create_face(
  indices: &mut Vec<u16>,
  colors: &mut Vec<Vector4<u8>>,
  p1: u16,
  p2: u16,
  p3: u16,
  p4: u16,
  color: Vector4<u8>,
) {
  [p1, p4, p2, p2, p4, p3].iter().for_each(|i| {
    indices.push(*i);
    colors.insert((*i) as usize, color)
  });
}
