use std::collections::HashMap;

use gaiku_common::{
  mint::{Vector3, Vector4},
  Baker, Chunk, Chunkify, Mesh,
};

pub struct GreedyMeshingBaker;

impl Baker for GreedyMeshingBaker {
  fn bake(chunk: &Chunk) -> Option<Mesh> {
    let mut indices = vec![];
    let mut vertices_cache: HashMap<String, (Vector3<f32>, u16)> = HashMap::new();
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

    let dimension = [chunk.width(), chunk.width(), chunk.height()];

    // Sweep over each axis (X, Y and Z)
    for axis in 0..3 {
      let mut i = 0;
      let u = (axis + 1) % 3;
      let v = (axis + 2) % 3;
      let mut position: [i32; 3] = [0; 3];
      let mut direction: [usize; 3] = [0; 3];

      let mut mask = vec![false; dimension[u] * dimension[v]];
      position[axis] = -1;
      direction[axis] = 1;

      // Check each slice of the chunk one at the time
      while position[axis] < dimension[axis] as i32 {
        let mut compute_mask = 0;

        for _v in 0..dimension[v] {
          position[v] = _v as i32;
          for _u in 0..dimension[u] {
            position[u] = _u as i32;
            // q determines the direction (x, y or z) that we are searching

            let block_current = if position[axis] >= 0 {
              chunk.get(
                position[0] as usize,
                position[1] as usize,
                position[2] as usize,
              ) > 0
            } else {
              false
            };
            println!(
              "{} + {} = {}",
              position[axis],
              dimension[axis],
              position[axis] + dimension[axis] as i32
            );
            let block_compare = if position[axis] + dimension[axis] as i32 >= 0
              && position[axis] < dimension[axis] as i32 - 1
            {
              chunk.get(
                (position[0] + direction[0] as i32) as usize,
                (position[1] + direction[1] as i32) as usize,
                (position[2] + direction[2] as i32) as usize,
              ) > 0
            } else {
              false
            };

            // The mask is set to true if there is a visible face between two blocks,
            // i.e. both aren't empty and both aren't blocks
            mask[compute_mask] = block_current != block_compare;
            compute_mask += 1;
          }
        }

        position[axis] += 1;
        compute_mask = 0;

        // Generate a mesh from the mask using lexicographic ordering,
        // by looping over each block in this slice of the chunk
        for j in 0..dimension[v] {
          while i < dimension[u] {
            if mask[compute_mask] {
              // Compute the width of this quad and store it in `width`
              // this is done by searching along the current axis until mask[compute_mask + width] is false
              let mut width = 1;
              let mut height = 1;

              while i + width < dimension[u] && mask[compute_mask + width] {
                width += 1;
              }

              // Compute the height of this quad and store it in `height`
              // This is done by checking every block next to this row (range 0 to width) is also part of the mask.
              // For example, if width is 5 we currently have a quad of dimension 1 x 5. To reduce triangle count,
              // greedy meshing will attempt to expand this quad out to chunk_size x 5, but will stop if it reaches a hole in the mask.
              'outer: while height < dimension[v] {
                // Check each block next to this quad
                for k in 0..width {
                  // If there's a hole in the mask, exit
                  if !mask[compute_mask + k + height * dimension[u]] {
                    break 'outer;
                  }
                }

                height += 1;
              }

              position[u] = i as i32;
              position[v] = j as i32;

              // axis_u and axis_v determine the size and orientation of this face
              let mut axis_u = [0; 3];
              axis_u[u] = width;

              let mut axis_v = [0; 3];
              axis_v[v] = height;

              create_face(
                &mut indices,
                &mut colors,
                Self::index(
                  &mut vertices_cache,
                  to_vector3([
                    position[0] as usize,
                    position[1] as usize,
                    position[2] as usize,
                  ]),
                ),
                Self::index(
                  &mut vertices_cache,
                  to_vector3([
                    position[0] as usize + axis_u[0],
                    position[1] as usize + axis_u[1],
                    position[2] as usize + axis_u[2],
                  ]),
                ),
                Self::index(
                  &mut vertices_cache,
                  to_vector3([
                    position[0] as usize + axis_u[0] + axis_v[0],
                    position[1] as usize + axis_u[1] + axis_v[1],
                    position[2] as usize + axis_u[2] + axis_v[2],
                  ]),
                ),
                Self::index(
                  &mut vertices_cache,
                  to_vector3([
                    position[0] as usize + axis_v[0],
                    position[1] as usize + axis_v[1],
                    position[2] as usize + axis_v[2],
                  ]),
                ),
                [255, 255, 255, 255].into(),
              );

              // Clear this part of the mask, so we don't add duplicate faces
              for h in 0..height {
                for w in 0..width {
                  mask[compute_mask + w + h * dimension[u]] = false;
                }
              }

              // Increment counters and continue
              i += width;
              compute_mask += width;
            } else {
              i += 1;
              compute_mask += 1;
            }
          }
        }
      }
    }

    let mut vertices: Vec<Vector3<f32>> = vec![[0.0, 0.0, 0.0].into(); vertices_cache.len()];
    for (_, (vertex, index)) in vertices_cache {
      vertices[index as usize] = vertex;
    }

    println!("{} {}", vertices.len(), indices.len());

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

fn to_vector3(a: [usize; 3]) -> Vector3<f32> {
  [a[0] as f32, a[1] as f32, a[2] as f32].into()
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
