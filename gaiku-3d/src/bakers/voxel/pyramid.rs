use std::collections::HashMap;

use gaiku_common::{
  mint::{Vector3, Vector4},
  Baker, Chunk, Chunkify, Mesh,
};

pub struct PyramidBaker;

// TODO: Optimize, don't create faces between chunks if there's a non empty voxel
impl Baker for PyramidBaker {
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

    let half_x = chunk.width() / 2;
    let half_y = chunk.height() / 2;
    let half_z = chunk.depth() / 2;

    // Top
    piramid_loop(chunk.width(), chunk.depth(), half_y, |x, z, y| {
      create_faces(
        chunk,
        &mut vertices_cache,
        &mut indices,
        &mut colors,
        x_limit,
        y_limit,
        z_limit,
        x_limit - x,
        y_limit - y,
        z_limit - z,
      )
    });

    // Bottom
    piramid_loop(chunk.width(), chunk.depth(), half_y, |x, z, y| {
      create_faces(
        chunk,
        &mut vertices_cache,
        &mut indices,
        &mut colors,
        x_limit,
        y_limit,
        z_limit,
        x,
        y,
        z,
      )
    });

    // Left
    piramid_loop(chunk.height(), chunk.depth(), half_x, |y, z, x| {
      create_faces(
        chunk,
        &mut vertices_cache,
        &mut indices,
        &mut colors,
        x_limit,
        y_limit,
        z_limit,
        x_limit - x,
        y_limit - y,
        z_limit - z,
      )
    });

    // Left
    piramid_loop(chunk.height(), chunk.depth(), half_x, |y, z, x| {
      create_faces(
        chunk,
        &mut vertices_cache,
        &mut indices,
        &mut colors,
        x_limit,
        y_limit,
        z_limit,
        x,
        y,
        z,
      )
    });

    // Front
    piramid_loop(chunk.width(), chunk.height(), half_z, |x, y, z| {
      create_faces(
        chunk,
        &mut vertices_cache,
        &mut indices,
        &mut colors,
        x_limit,
        y_limit,
        z_limit,
        x,
        y,
        z,
      )
    });

    // Back
    piramid_loop(chunk.width(), chunk.height(), half_z, |x, y, z| {
      create_faces(
        chunk,
        &mut vertices_cache,
        &mut indices,
        &mut colors,
        x_limit,
        y_limit,
        z_limit,
        x_limit - x,
        y_limit - y,
        z_limit - z,
      )
    });

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

fn piramid_loop<F>(max_x: usize, max_y: usize, max_level: usize, mut callback: F)
where
  F: FnMut(usize, usize, usize) -> u8,
{
  let mut current_level = 0;
  let mut is_solid = true;

  while current_level < max_level {
    for x in current_level..max_x - current_level {
      for y in current_level..max_y - current_level {
        let value = callback(x, y, current_level);

        if value == 0 {
          is_solid = false;
        }
      }
    }

    if is_solid {
      break;
    }

    current_level += 1;
  }
}

fn create_faces(
  chunk: &Chunk,
  vertices_cache: &mut HashMap<String, (Vector3<f32>, u16)>,
  indices: &mut Vec<u16>,
  colors: &mut Vec<Vector4<u8>>,
  x_limit: usize,
  y_limit: usize,
  z_limit: usize,
  x: usize,
  y: usize,
  z: usize,
) -> u8 {
  let (value, color) = chunk.get_with_color(x, y, z);
  if value == 0 {
    return 0;
  }

  let fx = x as f32;
  let fy = y as f32;
  let fz = z as f32;

  let top_left_back = index(vertices_cache, [fx - 0.5, fy + 0.5, fz - 0.5].into());
  let top_right_back = index(vertices_cache, [fx + 0.5, fy + 0.5, fz - 0.5].into());
  let top_right_front = index(vertices_cache, [fx + 0.5, fy + 0.5, fz + 0.5].into());
  let top_left_front = index(vertices_cache, [fx - 0.5, fy + 0.5, fz + 0.5].into());
  let bottom_left_back = index(vertices_cache, [fx - 0.5, fy - 0.5, fz - 0.5].into());
  let bottom_right_back = index(vertices_cache, [fx + 0.5, fy - 0.5, fz - 0.5].into());
  let bottom_right_front = index(vertices_cache, [fx + 0.5, fy - 0.5, fz + 0.5].into());
  let bottom_left_front = index(vertices_cache, [fx - 0.5, fy - 0.5, fz + 0.5].into());

  // Top
  if y == y_limit || chunk.is_air(x, y + 1, z) {
    create_face(
      indices,
      colors,
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
      indices,
      colors,
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
      indices,
      colors,
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
      indices,
      colors,
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
      indices,
      colors,
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
      indices,
      colors,
      top_right_back,
      top_left_back,
      bottom_left_back,
      bottom_right_back,
      color,
    );
  }

  value
}

fn index(vertices: &mut HashMap<String, (Vector3<f32>, u16)>, vertex: Vector3<f32>) -> u16 {
  let index = vertices.len();
  let key = format!("{:?}", vertex);
  vertices.entry(key).or_insert((vertex, index as u16)).1
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
