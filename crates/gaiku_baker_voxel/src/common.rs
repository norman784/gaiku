use gaiku_common::mint::Vector3;
use glam::Vec3;

use super::tables::{CORNER_TABLE, EDGE_TABLE, TRIANGLE_TABLE, UV_TABLE};

pub(crate) const EPSILON: f32 = 1e-4;

#[derive(Debug)]
pub(crate) struct GridCell {
  // 000
  // 100
  // 110
  // 010
  // 001
  // 101
  // 111
  // 011
  pub value: [f32; 8],
  pub point: [Vector3<f32>; 8],
}

impl GridCell {
  fn lerp(&self, index1: usize, index2: usize, isovalue: f32) -> [f32; 3] {
    let mut index1 = index1;
    let mut index2 = index2;

    if self.value[index2] < self.value[index1] {
      std::mem::swap(&mut index1, &mut index2);
    }

    let point1: Vec3 = self.point[index1].into();
    let point2: Vec3 = self.point[index2].into();

    if (point1 - point2).length() > EPSILON {
      let value1 = self.value[index1] as f32;
      let value2 = self.value[index2] as f32;

      if (isovalue - value1).abs() <= EPSILON {
        point1.into()
      } else if (isovalue - value2).abs() <= EPSILON {
        point2.into()
      } else if isovalue < value1 {
        unreachable!();
      } else if isovalue > value2 {
        unreachable!();
      } else {
        let weight = (isovalue - value1) / (value2 - value1);
        (point1 * weight + point2 * (1. - weight)).into()
      }
    } else {
      self.point[index1].into()
    }
  }

  // fn average(&self, verts: Vec<&[f32; 3]>) -> [f32; 3] {
  //   let mut result: [f32; 3] = [0., 0., 0.];
  //   for vert in verts.iter() {
  //     result[0] += vert[0];
  //     result[1] += vert[1];
  //     result[2] += vert[2];
  //   }
  //   result[0] /= verts.len() as f32;
  //   result[1] /= verts.len() as f32;
  //   result[2] /= verts.len() as f32;
  //   result
  // }

  fn mid_point(&self, indices: &[usize]) -> [f32; 3] {
    let len = indices.len();
    assert!(len > 0);
    let sum = indices
      .iter()
      .fold(Vec3::zero(), |acc, &i| Vec3::from(self.point[i]) + acc);
    (sum / len as f32).into()
  }

  pub(crate) fn polygonize(&self, isovalue: f32) -> Vec<([[f32; 3]; 3], [[f32; 2]; 3], i8)> {
    let mut cube_index = 0;
    let mut vertex_list = [[0.0, 0.0, 0.0]; 19];
    let mut triangles = vec![];

    if self.value[0] > isovalue {
      cube_index |= 1;
    }
    if self.value[1] > isovalue {
      cube_index |= 2;
    }
    if self.value[2] > isovalue {
      cube_index |= 4;
    }
    if self.value[3] > isovalue {
      cube_index |= 8;
    }
    if self.value[4] > isovalue {
      cube_index |= 16;
    }
    if self.value[5] > isovalue {
      cube_index |= 32;
    }
    if self.value[6] > isovalue {
      cube_index |= 64;
    }
    if self.value[7] > isovalue {
      cube_index |= 128;
    }

    // println!("cube_index: {}", cube_index);
    // println!("Values: {:?}", self.value);
    // let mut edges_to_do = vec![];
    // for i in 0..19 {
    //   if (EDGE_TABLE[cube_index] & 2__u32.pow(i as u32)) != 0 {
    //     edges_to_do.push(i);
    //   }
    // }
    // println!("Edges: {:?}", edges_to_do);

    if EDGE_TABLE[cube_index] == 0 {
      return vec![];
    }

    if (EDGE_TABLE[cube_index] & 2__u32.pow(0)) != 0 {
      vertex_list[0] = self.lerp(0, 1, isovalue);
    }

    if (EDGE_TABLE[cube_index] & 2__u32.pow(1)) != 0 {
      vertex_list[1] = self.lerp(1, 2, isovalue);
    }

    if (EDGE_TABLE[cube_index] & 2__u32.pow(2)) != 0 {
      vertex_list[2] = self.lerp(2, 3, isovalue);
    }

    if (EDGE_TABLE[cube_index] & 2__u32.pow(3)) != 0 {
      vertex_list[3] = self.lerp(3, 0, isovalue);
    }

    if (EDGE_TABLE[cube_index] & 2__u32.pow(4)) != 0 {
      vertex_list[4] = self.lerp(4, 5, isovalue);
    }

    if (EDGE_TABLE[cube_index] & 2__u32.pow(5)) != 0 {
      vertex_list[5] = self.lerp(5, 6, isovalue);
    }

    if (EDGE_TABLE[cube_index] & 2__u32.pow(6)) != 0 {
      vertex_list[6] = self.lerp(6, 7, isovalue);
    }

    if (EDGE_TABLE[cube_index] & 2__u32.pow(7)) != 0 {
      vertex_list[7] = self.lerp(7, 4, isovalue);
    }

    if (EDGE_TABLE[cube_index] & 2__u32.pow(8)) != 0 {
      vertex_list[8] = self.lerp(4, 0, isovalue);
    }

    if (EDGE_TABLE[cube_index] & 2__u32.pow(9)) != 0 {
      vertex_list[9] = self.lerp(5, 1, isovalue);
    }

    if (EDGE_TABLE[cube_index] & 2__u32.pow(10)) != 0 {
      vertex_list[10] = self.lerp(6, 2, isovalue);
    }

    if (EDGE_TABLE[cube_index] & 2__u32.pow(11)) != 0 {
      vertex_list[11] = self.lerp(7, 3, isovalue);
    }

    if (EDGE_TABLE[cube_index] & 2__u32.pow(12)) != 0 {
      let face_corners = [0, 1, 4, 5];
      let center = self.mid_point(&face_corners);
      let x = maybe_average(&[0, 4], cube_index, &vertex_list).unwrap_or(center)[0];

      let z = maybe_average(&[8, 9], cube_index, &vertex_list).unwrap_or(center)[2];

      let y = center[1];

      vertex_list[12] = [x, y, z];
    }

    if (EDGE_TABLE[cube_index] & 2__u32.pow(13)) != 0 {
      let face_corners = [1, 2, 5, 6];
      let center = self.mid_point(&face_corners);

      let z = maybe_average(&[9, 10], cube_index, &vertex_list).unwrap_or(center)[2];

      let y = maybe_average(&[1, 5], cube_index, &vertex_list).unwrap_or(center)[1];

      let x = center[0];

      vertex_list[13] = [x, y, z];
    }

    if (EDGE_TABLE[cube_index] & 2__u32.pow(14)) != 0 {
      let face_corners = [2, 3, 6, 7];
      let center = self.mid_point(&face_corners);

      let x = maybe_average(&[2, 6], cube_index, &vertex_list).unwrap_or(center)[0];

      let z = maybe_average(&[10, 11], cube_index, &vertex_list).unwrap_or(center)[2];

      let y = center[1];

      vertex_list[14] = [x, y, z];
    }

    if (EDGE_TABLE[cube_index] & 2__u32.pow(15)) != 0 {
      let face_corners = [0, 3, 4, 7];
      let center = self.mid_point(&face_corners);

      let z = maybe_average(&[8, 11], cube_index, &vertex_list).unwrap_or(center)[2];

      let y = maybe_average(&[3, 7], cube_index, &vertex_list).unwrap_or(center)[1];

      let x = center[0];

      vertex_list[15] = [x, y, z];
    }

    if (EDGE_TABLE[cube_index] & 2__u32.pow(16)) != 0 {
      let face_corners = [0, 1, 2, 3];
      let center = self.mid_point(&face_corners);

      let x = maybe_average(&[0, 2], cube_index, &vertex_list).unwrap_or(center)[0];

      let y = maybe_average(&[1, 3], cube_index, &vertex_list).unwrap_or(center)[1];

      let z = center[2];

      vertex_list[16] = [x, y, z];
    }

    if (EDGE_TABLE[cube_index] & 2__u32.pow(17)) != 0 {
      let face_corners = [4, 5, 6, 7];
      let center = self.mid_point(&face_corners);

      let x = maybe_average(&[4, 6], cube_index, &vertex_list).unwrap_or(center)[0];

      let y = maybe_average(&[5, 7], cube_index, &vertex_list).unwrap_or(center)[1];

      let z = center[2];

      vertex_list[17] = [x, y, z];
    }

    if (EDGE_TABLE[cube_index] & 2__u32.pow(18)) != 0 {
      let center = self.mid_point(&[0, 1, 2, 3, 4, 5, 6, 7]);
      let x = maybe_average(&[12, 14, 16, 17], cube_index, &vertex_list).unwrap_or(center)[0];

      let y = maybe_average(&[13, 15, 16, 17], cube_index, &vertex_list).unwrap_or(center)[1];

      let z = maybe_average(&[12, 13, 14, 15], cube_index, &vertex_list).unwrap_or(center)[2];

      vertex_list[18] = [x, y, z];
    }

    let mut i = 0;

    loop {
      if TRIANGLE_TABLE[cube_index][i] == -1 {
        break;
      }

      assert!(EDGE_TABLE[cube_index] & 2__u32.pow(TRIANGLE_TABLE[cube_index][i] as u32) != 0);
      assert!(EDGE_TABLE[cube_index] & 2__u32.pow(TRIANGLE_TABLE[cube_index][i + 1] as u32) != 0);
      assert!(EDGE_TABLE[cube_index] & 2__u32.pow(TRIANGLE_TABLE[cube_index][i + 2] as u32) != 0);

      let corner = CORNER_TABLE[cube_index][i];
      triangles.push((
        [
          vertex_list[TRIANGLE_TABLE[cube_index][i] as usize],
          vertex_list[TRIANGLE_TABLE[cube_index][i + 1] as usize],
          vertex_list[TRIANGLE_TABLE[cube_index][i + 2] as usize],
        ],
        [
          UV_TABLE[cube_index][i],
          UV_TABLE[cube_index][i + 1],
          UV_TABLE[cube_index][i + 2],
        ],
        corner,
      ));

      i += 3;
    }
    triangles
  }
}

fn maybe_average(
  axis_verts: &[usize],
  cube_index: usize,
  vertex_list: &[[f32; 3]; 19],
) -> Option<[f32; 3]> {
  let weight = axis_verts.iter().fold(0., |acc, &i| {
    if (EDGE_TABLE[cube_index] & 2__u32.pow(i as u32)) != 0 {
      acc + 1.
    } else {
      acc
    }
  });
  if weight > 0. {
    let sum = axis_verts.iter().fold(Vec3::zero(), |acc, &i| {
      if (EDGE_TABLE[cube_index] & 2__u32.pow(i as u32)) != 0 {
        Vec3::from(vertex_list[i]) + acc
      } else {
        acc
      }
    });
    Some((sum / weight).into())
  } else {
    None
  }
}

pub(crate) fn compute_normal(triangle: &[[f32; 3]; 3]) -> [f32; 3] {
  let v1: Vec3 = triangle[0].into();
  let v2: Vec3 = triangle[1].into();
  let v3: Vec3 = triangle[2].into();

  (v2 - v1).normalize().cross((v3 - v1).normalize()).into()
}
