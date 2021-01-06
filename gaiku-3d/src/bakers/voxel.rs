use std::collections::HashMap;

use gaiku_common::{
  mint::{Vector2, Vector3},
  Baker, Chunk, Chunkify, Mesh, Result, TextureAtlas2d,
};

pub struct VoxelBaker;

// Each vertex has the following data
struct VertexData {
  position: Vector3<usize>,
  normal: Vector3<i8>,
  uv: u8,
  index: u16,
}

impl VertexData {
  /// Check if we need to split the vertex because the normals differ
  pub fn is_same_normal(&self, norm: Vector3<i8>) -> bool {
    norm.x == self.normal.x && norm.y == self.normal.y && norm.z == self.normal.z
  }
}

// TODO: Optimize, don't create faces between chunks if there's a non empty voxel
impl Baker for VoxelBaker {
  fn bake(chunk: &Chunk, atlas: Option<&TextureAtlas2d>) -> Result<Option<Mesh>> {
    let mut indices = vec![];
    // Hash map in x, y, z coordinates to a list of verts at that coordinates
    let mut vertices: HashMap<(usize, usize, usize), Vec<VertexData>> = HashMap::new();

    let x_limit = chunk.width() - 1;
    let y_limit = chunk.height() - 1;
    let z_limit = chunk.depth() - 1;

    for x in 0..chunk.width() {
      for y in 0..chunk.height() {
        for z in 0..chunk.depth() {
          if chunk.is_air(x, y, z) {
            continue;
          }

          let atlas_index = chunk.get_index(x, y, z);

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
            create_face(
              &mut indices,
              &mut vertices,
              [
                top_left_back,
                top_right_back,
                top_right_front,
                top_left_front,
              ],
              Vector3 { x: 0, y: 1, z: 0 },
              atlas_index,
            );
          }

          // Bottom
          if y == 0 || (y > 0 && chunk.is_air(x, y - 1, z)) {
            create_face(
              &mut indices,
              &mut vertices,
              [
                bottom_right_back,
                bottom_left_back,
                bottom_left_front,
                bottom_right_front,
              ],
              Vector3 { x: 0, y: -1, z: 0 },
              atlas_index,
            );
          }

          // Left
          if x == 0 || (x > 0 && chunk.is_air(x - 1, y, z)) {
            create_face(
              &mut indices,
              &mut vertices,
              [
                top_left_back,
                top_left_front,
                bottom_left_front,
                bottom_left_back,
              ],
              Vector3 { x: -1, y: 0, z: 0 },
              atlas_index,
            );
          }

          // Right
          if x == x_limit || chunk.is_air(x + 1, y, z) {
            create_face(
              &mut indices,
              &mut vertices,
              [
                top_right_front,
                top_right_back,
                bottom_right_back,
                bottom_right_front,
              ],
              Vector3 { x: 1, y: 0, z: 0 },
              atlas_index,
            );
          }

          // Front
          if z == z_limit || chunk.is_air(x, y, z + 1) {
            create_face(
              &mut indices,
              &mut vertices,
              [
                top_left_front,
                top_right_front,
                bottom_right_front,
                bottom_left_front,
              ],
              Vector3 { x: 0, y: 0, z: 1 },
              atlas_index,
            );
          }

          // Back
          if z == 0 || chunk.is_air(x, y, z - 1) {
            create_face(
              &mut indices,
              &mut vertices,
              [
                top_right_back,
                top_left_back,
                bottom_left_back,
                bottom_right_back,
              ],
              Vector3 { x: 0, y: 0, z: -1 },
              atlas_index,
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

    let uv: Vec<(Vector2<f32>, Vector2<f32>)> = all_verts
      .iter()
      .map(|v| match atlas {
        Some(atlas) => atlas.get_uv(v.uv),
        _ => ([0.0, 0.0].into(), [0.0, 0.0].into()),
      })
      .collect();

    if !indices.is_empty() {
      Ok(Some(Mesh {
        indices,
        vertices,
        normals,
        uv,
        tangents: vec![],
      }))
    } else {
      Ok(None)
    }
  }
}

/// Either get the vertex at this position or insert one.
/// Only returns an old vertex if the position normal and color are the same
/// as the requested one
fn get_or_insert(
  cache: &mut HashMap<(usize, usize, usize), Vec<VertexData>>,
  position: (usize, usize, usize),
  uv: u8,
  normal: Vector3<i8>,
) -> u16 {
  // Get all verts at this position
  let verts = &mut cache.entry(position).or_insert_with(Vec::new);

  // Check each vert at this position to see if its valid.
  // This loop will only ever have 6 vertexes max
  for i in 0..verts.len() {
    let vert = &verts[i];
    if vert.is_same_normal(normal) && vert.uv == uv {
      // If there is already a valid vertex then return it
      return vert.index;
    }
  }

  // If not we must make a new one
  let next_index = cache.values().fold(0, |acc, v| acc + v.len()) as u16;
  let new_vert = VertexData {
    position: Vector3 {
      x: position.0,
      y: position.1,
      z: position.2,
    },
    normal,
    uv,
    index: next_index,
  };

  let verts = &mut cache.entry(position).or_insert_with(Vec::new);
  verts.push(new_vert);

  next_index
}

/// Create the face and insert the vertexes into the cache
fn create_face(
  indices: &mut Vec<u16>,
  cache: &mut HashMap<(usize, usize, usize), Vec<VertexData>>,
  p: [(usize, usize, usize); 4],
  normal: Vector3<i8>,
  uv: u8,
) {
  [p[0], p[3], p[1], p[1], p[3], p[2]].iter().for_each(|p| {
    let index = get_or_insert(cache, *p, uv, normal);
    indices.push(index);
  });
}
