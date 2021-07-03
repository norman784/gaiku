use gaiku_common::mint::Vector3;
use glam::Vec3;
use std::collections::HashMap;

pub(crate) const EPSILON: f32 = 1e-4;

use super::tables::{
  CORNER_TABLE, EDGE_TABLE, ORDINARY_EDGE_LEN, SPECIAL_BARYS, SPECIAL_EDGES, SPECIAL_EDGE_LEN,
  TRIANGLE_TABLE, UV_TABLE,
};

#[derive(Debug)]
pub(crate) struct GridCell {
  pub value: [f32; 8],
  pub point: [Vector3<f32>; 8],
}

impl GridCell {
  fn lerp(&self, index1: usize, index2: usize, isolevel: f32) -> [f32; 3] {
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

      if (isolevel - value1).abs() <= EPSILON {
        point1.into()
      } else if (isolevel - value2).abs() <= EPSILON {
        point2.into()
      } else if isolevel < value1 {
        unreachable!();
      } else if isolevel > value2 {
        unreachable!();
      } else {
        let weight = (isolevel - value1) / (value2 - value1);
        (point1 * weight + point2 * (1. - weight)).into()
      }
    } else {
      self.point[index1].into()
    }
  }

  pub(crate) fn polygonize(&self, isolevel: f32) -> Vec<([[f32; 3]; 3], [[f32; 2]; 3], i8)> {
    let mut cube_index = 0;
    let mut vertex_list = [[0.0, 0.0, 0.0]; 12];
    let mut special_list: HashMap<usize, [f32; 3]> = Default::default();
    let mut special_bary_list: HashMap<usize, [f32; 3]> = Default::default();
    let mut triangles = vec![];

    if self.value[0] < isolevel {
      cube_index |= 1;
    }
    if self.value[1] < isolevel {
      cube_index |= 2;
    }
    if self.value[2] < isolevel {
      cube_index |= 4;
    }
    if self.value[3] < isolevel {
      cube_index |= 8;
    }
    if self.value[4] < isolevel {
      cube_index |= 16;
    }
    if self.value[5] < isolevel {
      cube_index |= 32;
    }
    if self.value[6] < isolevel {
      cube_index |= 64;
    }
    if self.value[7] < isolevel {
      cube_index |= 128;
    }

    if EDGE_TABLE[cube_index] == 0 {
      return vec![];
    }

    if (EDGE_TABLE[cube_index] & 1) != 0 {
      vertex_list[0] = self.lerp(0, 1, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 2) != 0 {
      vertex_list[1] = self.lerp(1, 2, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 4) != 0 {
      vertex_list[2] = self.lerp(2, 3, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 8) != 0 {
      vertex_list[3] = self.lerp(3, 0, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 16) != 0 {
      vertex_list[4] = self.lerp(4, 5, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 32) != 0 {
      vertex_list[5] = self.lerp(5, 6, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 64) != 0 {
      vertex_list[6] = self.lerp(6, 7, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 128) != 0 {
      vertex_list[7] = self.lerp(7, 4, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 256) != 0 {
      vertex_list[8] = self.lerp(0, 4, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 512) != 0 {
      vertex_list[9] = self.lerp(1, 5, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 1024) != 0 {
      vertex_list[10] = self.lerp(2, 6, isolevel);
    }

    if (EDGE_TABLE[cube_index] & 2048) != 0 {
      vertex_list[11] = self.lerp(3, 7, isolevel);
    }

    let mut i = 0;

    loop {
      if TRIANGLE_TABLE[cube_index][i] == -1 {
        break;
      }

      let a = TRIANGLE_TABLE[cube_index][i] as usize;
      let b = TRIANGLE_TABLE[cube_index][i + 1] as usize;
      let c = TRIANGLE_TABLE[cube_index][i + 2] as usize;

      let v1 = if a < ORDINARY_EDGE_LEN {
        vertex_list[a]
      } else if a < ORDINARY_EDGE_LEN + SPECIAL_EDGE_LEN {
        let a_s = a - ORDINARY_EDGE_LEN;
        special_list
          .entry(a_s)
          .or_insert_with(|| {
            let (i, j, w1) = SPECIAL_EDGES[a_s];
            let v1: Vec3 = vertex_list[i].into();
            let v2: Vec3 = vertex_list[j].into();
            let w2 = 1. - w1;
            let res: [f32; 3] = (v1 * w1 + v2 * w2).into();
            res
          })
          .clone()
      } else {
        let a_s = a - ORDINARY_EDGE_LEN - SPECIAL_EDGE_LEN;
        special_bary_list
          .entry(a_s)
          .or_insert_with(|| {
            let (i, j, k, w1, w2) = SPECIAL_BARYS[a_s];
            let v1: Vec3 = vertex_list[i].into();
            let v2: Vec3 = vertex_list[j].into();
            let v3: Vec3 = vertex_list[k].into();
            let w3 = 1. - w1 - w2;
            let res: [f32; 3] = (v1 * w1 + v2 * w2 + v3 * w3).into();
            res
          })
          .clone()
      };

      let v2 = if b < ORDINARY_EDGE_LEN {
        vertex_list[b]
      } else if b < ORDINARY_EDGE_LEN + SPECIAL_EDGE_LEN {
        let b_s = b - ORDINARY_EDGE_LEN;
        special_list
          .entry(b_s)
          .or_insert_with(|| {
            let (i, j, w1) = SPECIAL_EDGES[b_s];
            let v1: Vec3 = vertex_list[i].into();
            let v2: Vec3 = vertex_list[j].into();
            let w2 = 1. - w1;
            let res: [f32; 3] = (v1 * w1 + v2 * w2).into();
            res
          })
          .clone()
      } else {
        let b_s = b - ORDINARY_EDGE_LEN - SPECIAL_EDGE_LEN;
        special_bary_list
          .entry(b_s)
          .or_insert_with(|| {
            let (i, j, k, w1, w2) = SPECIAL_BARYS[b_s];
            let v1: Vec3 = vertex_list[i].into();
            let v2: Vec3 = vertex_list[j].into();
            let v3: Vec3 = vertex_list[k].into();
            let w3 = 1. - w1 - w2;
            let res: [f32; 3] = (v1 * w1 + v2 * w2 + v3 * w3).into();
            res
          })
          .clone()
      };

      let v3 = if c < ORDINARY_EDGE_LEN {
        vertex_list[c]
      } else if c < ORDINARY_EDGE_LEN + SPECIAL_EDGE_LEN {
        let c_s = c - ORDINARY_EDGE_LEN;
        special_list
          .entry(c_s)
          .or_insert_with(|| {
            let (i, j, w1) = SPECIAL_EDGES[c_s];
            let v1: Vec3 = vertex_list[i].into();
            let v2: Vec3 = vertex_list[j].into();
            let w2 = 1. - w1;
            let res: [f32; 3] = (v1 * w1 + v2 * w2).into();
            res
          })
          .clone()
      } else {
        let c_s = c - ORDINARY_EDGE_LEN - SPECIAL_EDGE_LEN;
        special_bary_list
          .entry(c_s)
          .or_insert_with(|| {
            let (i, j, k, w1, w2) = SPECIAL_BARYS[c_s];
            let v1: Vec3 = vertex_list[i].into();
            let v2: Vec3 = vertex_list[j].into();
            let v3: Vec3 = vertex_list[k].into();
            let w3 = 1. - w1 - w2;
            let res: [f32; 3] = (v1 * w1 + v2 * w2 + v3 * w3).into();
            res
          })
          .clone()
      };

      let corner = CORNER_TABLE[cube_index][i];
      triangles.push((
        [v1, v2, v3],
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

pub(crate) fn compute_normal(triangle: &[[f32; 3]; 3]) -> [f32; 3] {
  let v1: Vec3 = triangle[0].into();
  let v2: Vec3 = triangle[1].into();
  let v3: Vec3 = triangle[2].into();

  (v2 - v1).normalize().cross((v3 - v1).normalize()).into()
}
