// This alters the marching cube table so that it includes
// the needed extra verts and faces so that we can apply
// the atlases and uvs
//
// The algorithm used here is kept as general as possible
// so that it will work on other grid based surfacing
// algorithms
use itertools::Itertools;
use nalgebra::{Vector2, Vector3};
use std::{collections::HashSet, convert::TryInto};

type Vec3 = Vector3<f32>;
type Vec2 = Vector2<f32>;

const EPSILON: f32 = 1e-4;

pub const EDGE_TABLE: [i32; 256] = [
  0x0, 0x109, 0x203, 0x30a, 0x406, 0x50f, 0x605, 0x70c, 0x80c, 0x905, 0xa0f, 0xb06, 0xc0a, 0xd03,
  0xe09, 0xf00, 0x190, 0x99, 0x393, 0x29a, 0x596, 0x49f, 0x795, 0x69c, 0x99c, 0x895, 0xb9f, 0xa96,
  0xd9a, 0xc93, 0xf99, 0xe90, 0x230, 0x339, 0x33, 0x13a, 0x636, 0x73f, 0x435, 0x53c, 0xa3c, 0xb35,
  0x83f, 0x936, 0xe3a, 0xf33, 0xc39, 0xd30, 0x3a0, 0x2a9, 0x1a3, 0xaa, 0x7a6, 0x6af, 0x5a5, 0x4ac,
  0xbac, 0xaa5, 0x9af, 0x8a6, 0xfaa, 0xea3, 0xda9, 0xca0, 0x460, 0x569, 0x663, 0x76a, 0x66, 0x16f,
  0x265, 0x36c, 0xc6c, 0xd65, 0xe6f, 0xf66, 0x86a, 0x963, 0xa69, 0xb60, 0x5f0, 0x4f9, 0x7f3, 0x6fa,
  0x1f6, 0xff, 0x3f5, 0x2fc, 0xdfc, 0xcf5, 0xfff, 0xef6, 0x9fa, 0x8f3, 0xbf9, 0xaf0, 0x650, 0x759,
  0x453, 0x55a, 0x256, 0x35f, 0x55, 0x15c, 0xe5c, 0xf55, 0xc5f, 0xd56, 0xa5a, 0xb53, 0x859, 0x950,
  0x7c0, 0x6c9, 0x5c3, 0x4ca, 0x3c6, 0x2cf, 0x1c5, 0xcc, 0xfcc, 0xec5, 0xdcf, 0xcc6, 0xbca, 0xac3,
  0x9c9, 0x8c0, 0x8c0, 0x9c9, 0xac3, 0xbca, 0xcc6, 0xdcf, 0xec5, 0xfcc, 0xcc, 0x1c5, 0x2cf, 0x3c6,
  0x4ca, 0x5c3, 0x6c9, 0x7c0, 0x950, 0x859, 0xb53, 0xa5a, 0xd56, 0xc5f, 0xf55, 0xe5c, 0x15c, 0x55,
  0x35f, 0x256, 0x55a, 0x453, 0x759, 0x650, 0xaf0, 0xbf9, 0x8f3, 0x9fa, 0xef6, 0xfff, 0xcf5, 0xdfc,
  0x2fc, 0x3f5, 0xff, 0x1f6, 0x6fa, 0x7f3, 0x4f9, 0x5f0, 0xb60, 0xa69, 0x963, 0x86a, 0xf66, 0xe6f,
  0xd65, 0xc6c, 0x36c, 0x265, 0x16f, 0x66, 0x76a, 0x663, 0x569, 0x460, 0xca0, 0xda9, 0xea3, 0xfaa,
  0x8a6, 0x9af, 0xaa5, 0xbac, 0x4ac, 0x5a5, 0x6af, 0x7a6, 0xaa, 0x1a3, 0x2a9, 0x3a0, 0xd30, 0xc39,
  0xf33, 0xe3a, 0x936, 0x83f, 0xb35, 0xa3c, 0x53c, 0x435, 0x73f, 0x636, 0x13a, 0x33, 0x339, 0x230,
  0xe90, 0xf99, 0xc93, 0xd9a, 0xa96, 0xb9f, 0x895, 0x99c, 0x69c, 0x795, 0x49f, 0x596, 0x29a, 0x393,
  0x99, 0x190, 0xf00, 0xe09, 0xd03, 0xc0a, 0xb06, 0xa0f, 0x905, 0x80c, 0x70c, 0x605, 0x50f, 0x406,
  0x30a, 0x203, 0x109, 0x0,
];

pub const TRIANGLE_TABLE: [[i8; 16]; 256] = [
  [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
  ],
  [0, 8, 3, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [0, 1, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [1, 8, 3, 9, 8, 1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [1, 2, 10, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [0, 8, 3, 1, 2, 10, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [9, 2, 10, 0, 2, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [2, 8, 3, 2, 10, 8, 10, 9, 8, -1, -1, -1, -1, -1, -1, -1],
  [3, 11, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [0, 11, 2, 8, 11, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [1, 9, 0, 2, 3, 11, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [1, 11, 2, 1, 9, 11, 9, 8, 11, -1, -1, -1, -1, -1, -1, -1],
  [3, 10, 1, 11, 10, 3, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [0, 10, 1, 0, 8, 10, 8, 11, 10, -1, -1, -1, -1, -1, -1, -1],
  [3, 9, 0, 3, 11, 9, 11, 10, 9, -1, -1, -1, -1, -1, -1, -1],
  [9, 8, 10, 10, 8, 11, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [4, 7, 8, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [4, 3, 0, 7, 3, 4, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [0, 1, 9, 8, 4, 7, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [4, 1, 9, 4, 7, 1, 7, 3, 1, -1, -1, -1, -1, -1, -1, -1],
  [1, 2, 10, 8, 4, 7, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [3, 4, 7, 3, 0, 4, 1, 2, 10, -1, -1, -1, -1, -1, -1, -1],
  [9, 2, 10, 9, 0, 2, 8, 4, 7, -1, -1, -1, -1, -1, -1, -1],
  [2, 10, 9, 2, 9, 7, 2, 7, 3, 7, 9, 4, -1, -1, -1, -1],
  [8, 4, 7, 3, 11, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [11, 4, 7, 11, 2, 4, 2, 0, 4, -1, -1, -1, -1, -1, -1, -1],
  [9, 0, 1, 8, 4, 7, 2, 3, 11, -1, -1, -1, -1, -1, -1, -1],
  [4, 7, 11, 9, 4, 11, 9, 11, 2, 9, 2, 1, -1, -1, -1, -1],
  [3, 10, 1, 3, 11, 10, 7, 8, 4, -1, -1, -1, -1, -1, -1, -1],
  [1, 11, 10, 1, 4, 11, 1, 0, 4, 7, 11, 4, -1, -1, -1, -1],
  [4, 7, 8, 9, 0, 11, 9, 11, 10, 11, 0, 3, -1, -1, -1, -1],
  [4, 7, 11, 4, 11, 9, 9, 11, 10, -1, -1, -1, -1, -1, -1, -1],
  [9, 5, 4, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [9, 5, 4, 0, 8, 3, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [0, 5, 4, 1, 5, 0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [8, 5, 4, 8, 3, 5, 3, 1, 5, -1, -1, -1, -1, -1, -1, -1],
  [1, 2, 10, 9, 5, 4, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [3, 0, 8, 1, 2, 10, 4, 9, 5, -1, -1, -1, -1, -1, -1, -1],
  [5, 2, 10, 5, 4, 2, 4, 0, 2, -1, -1, -1, -1, -1, -1, -1],
  [2, 10, 5, 3, 2, 5, 3, 5, 4, 3, 4, 8, -1, -1, -1, -1],
  [9, 5, 4, 2, 3, 11, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [0, 11, 2, 0, 8, 11, 4, 9, 5, -1, -1, -1, -1, -1, -1, -1],
  [0, 5, 4, 0, 1, 5, 2, 3, 11, -1, -1, -1, -1, -1, -1, -1],
  [2, 1, 5, 2, 5, 8, 2, 8, 11, 4, 8, 5, -1, -1, -1, -1],
  [10, 3, 11, 10, 1, 3, 9, 5, 4, -1, -1, -1, -1, -1, -1, -1],
  [4, 9, 5, 0, 8, 1, 8, 10, 1, 8, 11, 10, -1, -1, -1, -1],
  [5, 4, 0, 5, 0, 11, 5, 11, 10, 11, 0, 3, -1, -1, -1, -1],
  [5, 4, 8, 5, 8, 10, 10, 8, 11, -1, -1, -1, -1, -1, -1, -1],
  [9, 7, 8, 5, 7, 9, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [9, 3, 0, 9, 5, 3, 5, 7, 3, -1, -1, -1, -1, -1, -1, -1],
  [0, 7, 8, 0, 1, 7, 1, 5, 7, -1, -1, -1, -1, -1, -1, -1],
  [1, 5, 3, 3, 5, 7, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [9, 7, 8, 9, 5, 7, 10, 1, 2, -1, -1, -1, -1, -1, -1, -1],
  [10, 1, 2, 9, 5, 0, 5, 3, 0, 5, 7, 3, -1, -1, -1, -1],
  [8, 0, 2, 8, 2, 5, 8, 5, 7, 10, 5, 2, -1, -1, -1, -1],
  [2, 10, 5, 2, 5, 3, 3, 5, 7, -1, -1, -1, -1, -1, -1, -1],
  [7, 9, 5, 7, 8, 9, 3, 11, 2, -1, -1, -1, -1, -1, -1, -1],
  [9, 5, 7, 9, 7, 2, 9, 2, 0, 2, 7, 11, -1, -1, -1, -1],
  [2, 3, 11, 0, 1, 8, 1, 7, 8, 1, 5, 7, -1, -1, -1, -1],
  [11, 2, 1, 11, 1, 7, 7, 1, 5, -1, -1, -1, -1, -1, -1, -1],
  [9, 5, 8, 8, 5, 7, 10, 1, 3, 10, 3, 11, -1, -1, -1, -1],
  [5, 7, 0, 5, 0, 9, 7, 11, 0, 1, 0, 10, 11, 10, 0, -1],
  [11, 10, 0, 11, 0, 3, 10, 5, 0, 8, 0, 7, 5, 7, 0, -1],
  [11, 10, 5, 7, 11, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [10, 6, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [0, 8, 3, 5, 10, 6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [9, 0, 1, 5, 10, 6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [1, 8, 3, 1, 9, 8, 5, 10, 6, -1, -1, -1, -1, -1, -1, -1],
  [1, 6, 5, 2, 6, 1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [1, 6, 5, 1, 2, 6, 3, 0, 8, -1, -1, -1, -1, -1, -1, -1],
  [9, 6, 5, 9, 0, 6, 0, 2, 6, -1, -1, -1, -1, -1, -1, -1],
  [5, 9, 8, 5, 8, 2, 5, 2, 6, 3, 2, 8, -1, -1, -1, -1],
  [2, 3, 11, 10, 6, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [11, 0, 8, 11, 2, 0, 10, 6, 5, -1, -1, -1, -1, -1, -1, -1],
  [0, 1, 9, 2, 3, 11, 5, 10, 6, -1, -1, -1, -1, -1, -1, -1],
  [5, 10, 6, 1, 9, 2, 9, 11, 2, 9, 8, 11, -1, -1, -1, -1],
  [6, 3, 11, 6, 5, 3, 5, 1, 3, -1, -1, -1, -1, -1, -1, -1],
  [0, 8, 11, 0, 11, 5, 0, 5, 1, 5, 11, 6, -1, -1, -1, -1],
  [3, 11, 6, 0, 3, 6, 0, 6, 5, 0, 5, 9, -1, -1, -1, -1],
  [6, 5, 9, 6, 9, 11, 11, 9, 8, -1, -1, -1, -1, -1, -1, -1],
  [5, 10, 6, 4, 7, 8, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [4, 3, 0, 4, 7, 3, 6, 5, 10, -1, -1, -1, -1, -1, -1, -1],
  [1, 9, 0, 5, 10, 6, 8, 4, 7, -1, -1, -1, -1, -1, -1, -1],
  [10, 6, 5, 1, 9, 7, 1, 7, 3, 7, 9, 4, -1, -1, -1, -1],
  [6, 1, 2, 6, 5, 1, 4, 7, 8, -1, -1, -1, -1, -1, -1, -1],
  [1, 2, 5, 5, 2, 6, 3, 0, 4, 3, 4, 7, -1, -1, -1, -1],
  [8, 4, 7, 9, 0, 5, 0, 6, 5, 0, 2, 6, -1, -1, -1, -1],
  [7, 3, 9, 7, 9, 4, 3, 2, 9, 5, 9, 6, 2, 6, 9, -1],
  [3, 11, 2, 7, 8, 4, 10, 6, 5, -1, -1, -1, -1, -1, -1, -1],
  [5, 10, 6, 4, 7, 2, 4, 2, 0, 2, 7, 11, -1, -1, -1, -1],
  [0, 1, 9, 4, 7, 8, 2, 3, 11, 5, 10, 6, -1, -1, -1, -1],
  [9, 2, 1, 9, 11, 2, 9, 4, 11, 7, 11, 4, 5, 10, 6, -1],
  [8, 4, 7, 3, 11, 5, 3, 5, 1, 5, 11, 6, -1, -1, -1, -1],
  [5, 1, 11, 5, 11, 6, 1, 0, 11, 7, 11, 4, 0, 4, 11, -1],
  [0, 5, 9, 0, 6, 5, 0, 3, 6, 11, 6, 3, 8, 4, 7, -1],
  [6, 5, 9, 6, 9, 11, 4, 7, 9, 7, 11, 9, -1, -1, -1, -1],
  [10, 4, 9, 6, 4, 10, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [4, 10, 6, 4, 9, 10, 0, 8, 3, -1, -1, -1, -1, -1, -1, -1],
  [10, 0, 1, 10, 6, 0, 6, 4, 0, -1, -1, -1, -1, -1, -1, -1],
  [8, 3, 1, 8, 1, 6, 8, 6, 4, 6, 1, 10, -1, -1, -1, -1],
  [1, 4, 9, 1, 2, 4, 2, 6, 4, -1, -1, -1, -1, -1, -1, -1],
  [3, 0, 8, 1, 2, 9, 2, 4, 9, 2, 6, 4, -1, -1, -1, -1],
  [0, 2, 4, 4, 2, 6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [8, 3, 2, 8, 2, 4, 4, 2, 6, -1, -1, -1, -1, -1, -1, -1],
  [10, 4, 9, 10, 6, 4, 11, 2, 3, -1, -1, -1, -1, -1, -1, -1],
  [0, 8, 2, 2, 8, 11, 4, 9, 10, 4, 10, 6, -1, -1, -1, -1],
  [3, 11, 2, 0, 1, 6, 0, 6, 4, 6, 1, 10, -1, -1, -1, -1],
  [6, 4, 1, 6, 1, 10, 4, 8, 1, 2, 1, 11, 8, 11, 1, -1],
  [9, 6, 4, 9, 3, 6, 9, 1, 3, 11, 6, 3, -1, -1, -1, -1],
  [8, 11, 1, 8, 1, 0, 11, 6, 1, 9, 1, 4, 6, 4, 1, -1],
  [3, 11, 6, 3, 6, 0, 0, 6, 4, -1, -1, -1, -1, -1, -1, -1],
  [6, 4, 8, 11, 6, 8, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [7, 10, 6, 7, 8, 10, 8, 9, 10, -1, -1, -1, -1, -1, -1, -1],
  [0, 7, 3, 0, 10, 7, 0, 9, 10, 6, 7, 10, -1, -1, -1, -1],
  [10, 6, 7, 1, 10, 7, 1, 7, 8, 1, 8, 0, -1, -1, -1, -1],
  [10, 6, 7, 10, 7, 1, 1, 7, 3, -1, -1, -1, -1, -1, -1, -1],
  [1, 2, 6, 1, 6, 8, 1, 8, 9, 8, 6, 7, -1, -1, -1, -1],
  [2, 6, 9, 2, 9, 1, 6, 7, 9, 0, 9, 3, 7, 3, 9, -1],
  [7, 8, 0, 7, 0, 6, 6, 0, 2, -1, -1, -1, -1, -1, -1, -1],
  [7, 3, 2, 6, 7, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [2, 3, 11, 10, 6, 8, 10, 8, 9, 8, 6, 7, -1, -1, -1, -1],
  [2, 0, 7, 2, 7, 11, 0, 9, 7, 6, 7, 10, 9, 10, 7, -1],
  [1, 8, 0, 1, 7, 8, 1, 10, 7, 6, 7, 10, 2, 3, 11, -1],
  [11, 2, 1, 11, 1, 7, 10, 6, 1, 6, 7, 1, -1, -1, -1, -1],
  [8, 9, 6, 8, 6, 7, 9, 1, 6, 11, 6, 3, 1, 3, 6, -1],
  [0, 9, 1, 11, 6, 7, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [7, 8, 0, 7, 0, 6, 3, 11, 0, 11, 6, 0, -1, -1, -1, -1],
  [7, 11, 6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [7, 6, 11, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [3, 0, 8, 11, 7, 6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [0, 1, 9, 11, 7, 6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [8, 1, 9, 8, 3, 1, 11, 7, 6, -1, -1, -1, -1, -1, -1, -1],
  [10, 1, 2, 6, 11, 7, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [1, 2, 10, 3, 0, 8, 6, 11, 7, -1, -1, -1, -1, -1, -1, -1],
  [2, 9, 0, 2, 10, 9, 6, 11, 7, -1, -1, -1, -1, -1, -1, -1],
  [6, 11, 7, 2, 10, 3, 10, 8, 3, 10, 9, 8, -1, -1, -1, -1],
  [7, 2, 3, 6, 2, 7, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [7, 0, 8, 7, 6, 0, 6, 2, 0, -1, -1, -1, -1, -1, -1, -1],
  [2, 7, 6, 2, 3, 7, 0, 1, 9, -1, -1, -1, -1, -1, -1, -1],
  [1, 6, 2, 1, 8, 6, 1, 9, 8, 8, 7, 6, -1, -1, -1, -1],
  [10, 7, 6, 10, 1, 7, 1, 3, 7, -1, -1, -1, -1, -1, -1, -1],
  [10, 7, 6, 1, 7, 10, 1, 8, 7, 1, 0, 8, -1, -1, -1, -1],
  [0, 3, 7, 0, 7, 10, 0, 10, 9, 6, 10, 7, -1, -1, -1, -1],
  [7, 6, 10, 7, 10, 8, 8, 10, 9, -1, -1, -1, -1, -1, -1, -1],
  [6, 8, 4, 11, 8, 6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [3, 6, 11, 3, 0, 6, 0, 4, 6, -1, -1, -1, -1, -1, -1, -1],
  [8, 6, 11, 8, 4, 6, 9, 0, 1, -1, -1, -1, -1, -1, -1, -1],
  [9, 4, 6, 9, 6, 3, 9, 3, 1, 11, 3, 6, -1, -1, -1, -1],
  [6, 8, 4, 6, 11, 8, 2, 10, 1, -1, -1, -1, -1, -1, -1, -1],
  [1, 2, 10, 3, 0, 11, 0, 6, 11, 0, 4, 6, -1, -1, -1, -1],
  [4, 11, 8, 4, 6, 11, 0, 2, 9, 2, 10, 9, -1, -1, -1, -1],
  [10, 9, 3, 10, 3, 2, 9, 4, 3, 11, 3, 6, 4, 6, 3, -1],
  [8, 2, 3, 8, 4, 2, 4, 6, 2, -1, -1, -1, -1, -1, -1, -1],
  [0, 4, 2, 4, 6, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [1, 9, 0, 2, 3, 4, 2, 4, 6, 4, 3, 8, -1, -1, -1, -1],
  [1, 9, 4, 1, 4, 2, 2, 4, 6, -1, -1, -1, -1, -1, -1, -1],
  [8, 1, 3, 8, 6, 1, 8, 4, 6, 6, 10, 1, -1, -1, -1, -1],
  [10, 1, 0, 10, 0, 6, 6, 0, 4, -1, -1, -1, -1, -1, -1, -1],
  [4, 6, 3, 4, 3, 8, 6, 10, 3, 0, 3, 9, 10, 9, 3, -1],
  [10, 9, 4, 6, 10, 4, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [4, 9, 5, 7, 6, 11, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [0, 8, 3, 4, 9, 5, 11, 7, 6, -1, -1, -1, -1, -1, -1, -1],
  [5, 0, 1, 5, 4, 0, 7, 6, 11, -1, -1, -1, -1, -1, -1, -1],
  [11, 7, 6, 8, 3, 4, 3, 5, 4, 3, 1, 5, -1, -1, -1, -1],
  [9, 5, 4, 10, 1, 2, 7, 6, 11, -1, -1, -1, -1, -1, -1, -1],
  [6, 11, 7, 1, 2, 10, 0, 8, 3, 4, 9, 5, -1, -1, -1, -1],
  [7, 6, 11, 5, 4, 10, 4, 2, 10, 4, 0, 2, -1, -1, -1, -1],
  [3, 4, 8, 3, 5, 4, 3, 2, 5, 10, 5, 2, 11, 7, 6, -1],
  [7, 2, 3, 7, 6, 2, 5, 4, 9, -1, -1, -1, -1, -1, -1, -1],
  [9, 5, 4, 0, 8, 6, 0, 6, 2, 6, 8, 7, -1, -1, -1, -1],
  [3, 6, 2, 3, 7, 6, 1, 5, 0, 5, 4, 0, -1, -1, -1, -1],
  [6, 2, 8, 6, 8, 7, 2, 1, 8, 4, 8, 5, 1, 5, 8, -1],
  [9, 5, 4, 10, 1, 6, 1, 7, 6, 1, 3, 7, -1, -1, -1, -1],
  [1, 6, 10, 1, 7, 6, 1, 0, 7, 8, 7, 0, 9, 5, 4, -1],
  [4, 0, 10, 4, 10, 5, 0, 3, 10, 6, 10, 7, 3, 7, 10, -1],
  [7, 6, 10, 7, 10, 8, 5, 4, 10, 4, 8, 10, -1, -1, -1, -1],
  [6, 9, 5, 6, 11, 9, 11, 8, 9, -1, -1, -1, -1, -1, -1, -1],
  [3, 6, 11, 0, 6, 3, 0, 5, 6, 0, 9, 5, -1, -1, -1, -1],
  [0, 11, 8, 0, 5, 11, 0, 1, 5, 5, 6, 11, -1, -1, -1, -1],
  [6, 11, 3, 6, 3, 5, 5, 3, 1, -1, -1, -1, -1, -1, -1, -1],
  [1, 2, 10, 9, 5, 11, 9, 11, 8, 11, 5, 6, -1, -1, -1, -1],
  [0, 11, 3, 0, 6, 11, 0, 9, 6, 5, 6, 9, 1, 2, 10, -1],
  [11, 8, 5, 11, 5, 6, 8, 0, 5, 10, 5, 2, 0, 2, 5, -1],
  [6, 11, 3, 6, 3, 5, 2, 10, 3, 10, 5, 3, -1, -1, -1, -1],
  [5, 8, 9, 5, 2, 8, 5, 6, 2, 3, 8, 2, -1, -1, -1, -1],
  [9, 5, 6, 9, 6, 0, 0, 6, 2, -1, -1, -1, -1, -1, -1, -1],
  [1, 5, 8, 1, 8, 0, 5, 6, 8, 3, 8, 2, 6, 2, 8, -1],
  [1, 5, 6, 2, 1, 6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [1, 3, 6, 1, 6, 10, 3, 8, 6, 5, 6, 9, 8, 9, 6, -1],
  [10, 1, 0, 10, 0, 6, 9, 5, 0, 5, 6, 0, -1, -1, -1, -1],
  [0, 3, 8, 5, 6, 10, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [10, 5, 6, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [11, 5, 10, 7, 5, 11, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [11, 5, 10, 11, 7, 5, 8, 3, 0, -1, -1, -1, -1, -1, -1, -1],
  [5, 11, 7, 5, 10, 11, 1, 9, 0, -1, -1, -1, -1, -1, -1, -1],
  [10, 7, 5, 10, 11, 7, 9, 8, 1, 8, 3, 1, -1, -1, -1, -1],
  [11, 1, 2, 11, 7, 1, 7, 5, 1, -1, -1, -1, -1, -1, -1, -1],
  [0, 8, 3, 1, 2, 7, 1, 7, 5, 7, 2, 11, -1, -1, -1, -1],
  [9, 7, 5, 9, 2, 7, 9, 0, 2, 2, 11, 7, -1, -1, -1, -1],
  [7, 5, 2, 7, 2, 11, 5, 9, 2, 3, 2, 8, 9, 8, 2, -1],
  [2, 5, 10, 2, 3, 5, 3, 7, 5, -1, -1, -1, -1, -1, -1, -1],
  [8, 2, 0, 8, 5, 2, 8, 7, 5, 10, 2, 5, -1, -1, -1, -1],
  [9, 0, 1, 5, 10, 3, 5, 3, 7, 3, 10, 2, -1, -1, -1, -1],
  [9, 8, 2, 9, 2, 1, 8, 7, 2, 10, 2, 5, 7, 5, 2, -1],
  [1, 3, 5, 3, 7, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [0, 8, 7, 0, 7, 1, 1, 7, 5, -1, -1, -1, -1, -1, -1, -1],
  [9, 0, 3, 9, 3, 5, 5, 3, 7, -1, -1, -1, -1, -1, -1, -1],
  [9, 8, 7, 5, 9, 7, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [5, 8, 4, 5, 10, 8, 10, 11, 8, -1, -1, -1, -1, -1, -1, -1],
  [5, 0, 4, 5, 11, 0, 5, 10, 11, 11, 3, 0, -1, -1, -1, -1],
  [0, 1, 9, 8, 4, 10, 8, 10, 11, 10, 4, 5, -1, -1, -1, -1],
  [10, 11, 4, 10, 4, 5, 11, 3, 4, 9, 4, 1, 3, 1, 4, -1],
  [2, 5, 1, 2, 8, 5, 2, 11, 8, 4, 5, 8, -1, -1, -1, -1],
  [0, 4, 11, 0, 11, 3, 4, 5, 11, 2, 11, 1, 5, 1, 11, -1],
  [0, 2, 5, 0, 5, 9, 2, 11, 5, 4, 5, 8, 11, 8, 5, -1],
  [9, 4, 5, 2, 11, 3, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [2, 5, 10, 3, 5, 2, 3, 4, 5, 3, 8, 4, -1, -1, -1, -1],
  [5, 10, 2, 5, 2, 4, 4, 2, 0, -1, -1, -1, -1, -1, -1, -1],
  [3, 10, 2, 3, 5, 10, 3, 8, 5, 4, 5, 8, 0, 1, 9, -1],
  [5, 10, 2, 5, 2, 4, 1, 9, 2, 9, 4, 2, -1, -1, -1, -1],
  [8, 4, 5, 8, 5, 3, 3, 5, 1, -1, -1, -1, -1, -1, -1, -1],
  [0, 4, 5, 1, 0, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [8, 4, 5, 8, 5, 3, 9, 0, 5, 0, 3, 5, -1, -1, -1, -1],
  [9, 4, 5, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [4, 11, 7, 4, 9, 11, 9, 10, 11, -1, -1, -1, -1, -1, -1, -1],
  [0, 8, 3, 4, 9, 7, 9, 11, 7, 9, 10, 11, -1, -1, -1, -1],
  [1, 10, 11, 1, 11, 4, 1, 4, 0, 7, 4, 11, -1, -1, -1, -1],
  [3, 1, 4, 3, 4, 8, 1, 10, 4, 7, 4, 11, 10, 11, 4, -1],
  [4, 11, 7, 9, 11, 4, 9, 2, 11, 9, 1, 2, -1, -1, -1, -1],
  [9, 7, 4, 9, 11, 7, 9, 1, 11, 2, 11, 1, 0, 8, 3, -1],
  [11, 7, 4, 11, 4, 2, 2, 4, 0, -1, -1, -1, -1, -1, -1, -1],
  [11, 7, 4, 11, 4, 2, 8, 3, 4, 3, 2, 4, -1, -1, -1, -1],
  [2, 9, 10, 2, 7, 9, 2, 3, 7, 7, 4, 9, -1, -1, -1, -1],
  [9, 10, 7, 9, 7, 4, 10, 2, 7, 8, 7, 0, 2, 0, 7, -1],
  [3, 7, 10, 3, 10, 2, 7, 4, 10, 1, 10, 0, 4, 0, 10, -1],
  [1, 10, 2, 8, 7, 4, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [4, 9, 1, 4, 1, 7, 7, 1, 3, -1, -1, -1, -1, -1, -1, -1],
  [4, 9, 1, 4, 1, 7, 0, 8, 1, 8, 7, 1, -1, -1, -1, -1],
  [4, 0, 3, 7, 4, 3, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [4, 8, 7, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [9, 10, 8, 10, 11, 8, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [3, 0, 9, 3, 9, 11, 11, 9, 10, -1, -1, -1, -1, -1, -1, -1],
  [0, 1, 10, 0, 10, 8, 8, 10, 11, -1, -1, -1, -1, -1, -1, -1],
  [3, 1, 10, 11, 3, 10, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [1, 2, 11, 1, 11, 9, 9, 11, 8, -1, -1, -1, -1, -1, -1, -1],
  [3, 0, 9, 3, 9, 11, 1, 2, 9, 2, 11, 9, -1, -1, -1, -1],
  [0, 2, 11, 8, 0, 11, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [3, 2, 11, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [2, 3, 8, 2, 8, 10, 10, 8, 9, -1, -1, -1, -1, -1, -1, -1],
  [9, 10, 2, 0, 9, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [2, 3, 8, 2, 8, 10, 0, 1, 8, 1, 10, 8, -1, -1, -1, -1],
  [1, 10, 2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [1, 3, 8, 9, 1, 8, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [0, 9, 1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [0, 3, 8, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1],
  [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
  ],
];

fn xy_to_barycentric(p: &Vec2, a: &Vec2, b: &Vec2, c: &Vec2) -> Vec3 {
  let l0: f32 = ((b[1] - c[1]) * (p[0] - c[0]) + (c[0] - b[0]) * (p[1] - c[1]))
    / ((b[1] - c[1]) * (a[0] - c[0]) + (c[0] - b[0]) * (a[1] - c[1]));
  let l1: f32 = ((c[1] - a[1]) * (p[0] - c[0]) + (a[0] - c[0]) * (p[1] - c[1]))
    / ((b[1] - c[1]) * (a[0] - c[0]) + (c[0] - b[0]) * (a[1] - c[1]));
  return [l0, l1, 1.0 - l0 - l1].into();
}

fn xyz_to_barycentric(p: &Vec3, a: &Vec3, b: &Vec3, c: &Vec3) -> Vec3 {
  // Project onto the triangles 2d plane
  let b_local = b - a;
  let c_local = c - a;
  let p_local = p - a;

  // by first finding the x and y axis of the triangle
  let u = b_local.normalize();
  let v = c_local.normalize();
  let n = v.cross(&u);
  let x_axis = u;
  let y_axis = x_axis.cross(&n);

  // Convert to 2d coords using these axis
  let p_2d: Vec2 = [p_local.dot(&x_axis), p_local.dot(&y_axis)].into();
  let a_2d: Vec2 = [0., 0.].into();
  let b_2d: Vec2 = [b_local.dot(&x_axis), b_local.dot(&y_axis)].into();
  let c_2d: Vec2 = [c_local.dot(&x_axis), c_local.dot(&y_axis)].into();

  // Now just standard barycentric
  return xy_to_barycentric(&p_2d, &a_2d, &b_2d, &c_2d);
}

#[derive(Debug, Clone)]
struct Plane {
  origin: Vec3,
  normal: Vec3,
}

#[derive(Debug, Clone)]
struct Line {
  origin: Vec3,
  direction: Vec3,
}

impl Line {
  fn from_points(a: &Vec3, b: &Vec3) -> Self {
    Self {
      origin: a.clone(),
      direction: (b - a).normalize(),
    }
  }

  fn plane_intersection(&self, plane: &Plane) -> Option<Vec3> {
    let origin_diff = self.origin - plane.origin;
    let cos_normal = self.direction.dot(&plane.normal);
    if (cos_normal - 0.).abs() < EPSILON {
      // Parallel lines are either complete miss
      // or every point hit
      // In both cases we don't want these results
      return None;
    }
    if origin_diff.norm() < EPSILON {
      return Some(self.origin.clone());
    }
    let diff_normal = origin_diff.dot(&plane.normal);
    let prod = diff_normal / cos_normal;
    return Some(self.origin - self.direction * prod);
  }

  #[allow(dead_code)]
  fn line_intersection(&self, line: &Self) -> Option<Vec3> {
    let origin_diff = self.origin - line.direction;
    let cos_normal = self.direction.dot(&line.direction);
    if (cos_normal - 1.).abs() < EPSILON {
      // Parallel lines are either complete miss
      // or every point hit
      // In both cases we don't want these results
      return None;
    }
    if origin_diff.norm() < EPSILON {
      return Some(self.origin.clone());
    }

    let c = self.origin;
    let d = line.origin;

    let e = self.direction;
    let f = line.direction;
    let g = d - c;

    // Is c or d on the other line
    if g.normalize().dot(&f).abs() < EPSILON {
      return Some(c.clone());
    }
    if g.normalize().dot(&e).abs() < EPSILON {
      return Some(d.clone());
    }

    let h = f.cross(&g).norm();
    let k = f.cross(&e).norm();

    if h < EPSILON || k < EPSILON {
      return None;
    }

    let l = h / k * e;

    let fxg = f.cross(&g);
    let fxe = f.cross(&e);

    if (fxg.normalize().dot(&fxe) - 1.0).abs() < EPSILON {
      return Some(c + l);
    } else if (fxg.normalize().dot(&fxe) + 1.0).abs() < EPSILON {
      return Some(c - l);
    } else {
      unreachable!();
    }
  }
}

#[derive(Debug, Clone)]
struct Edge {
  a: Vec3,
  b: Vec3,
}

impl Edge {
  fn contains(&self, point: &Vec3) -> bool {
    if (self.a - point).norm() < EPSILON {
      true
    } else if (self.b - point).norm() < EPSILON {
      true
    } else {
      let mins = [
        self.a[0].min(self.b[0]),
        self.a[1].min(self.b[1]),
        self.a[2].min(self.b[2]),
      ];
      let maxs = [
        self.a[0].max(self.b[0]),
        self.a[1].max(self.b[1]),
        self.a[2].max(self.b[2]),
      ];
      if ((point[0] - mins[0]).abs() < EPSILON
        || (point[0] - maxs[0]).abs() < EPSILON
        || (point[0] >= mins[0] && point[0] <= maxs[0]))
        && ((point[1] - mins[1]).abs() < EPSILON
          || (point[1] - maxs[1]).abs() < EPSILON
          || (point[1] >= mins[1] && point[1] <= maxs[1]))
        && ((point[2] - mins[2]).abs() < EPSILON
          || (point[2] - maxs[2]).abs() < EPSILON
          || (point[2] >= mins[2] && point[2] <= maxs[2]))
      {
        true
      } else {
        false
      }
    }
  }
  fn plane_intersection(&self, plane: &Plane) -> Option<Vec3> {
    let line = Line::from_points(&self.a, &self.b);
    if let Some(point) = line.plane_intersection(plane) {
      if (self.a - point).norm() < EPSILON {
        Some(self.a.clone())
      } else if (self.b - point).norm() < EPSILON {
        Some(self.b.clone())
      } else if self.contains(&point) {
        Some(point)
      } else {
        None
      }
    } else {
      None
    }
  }

  #[allow(dead_code)]
  fn edge_intersection(&self, edge: &Edge) -> Option<Vec3> {
    let me_line = Line::from_points(&self.a, &self.b);
    let edge_line = Line::from_points(&edge.a, &edge.b);
    if let Some(point) = me_line.line_intersection(&edge_line) {
      if (self.a - point).norm() < EPSILON {
        Some(self.a.clone())
      } else if (self.b - point).norm() < EPSILON {
        Some(self.b.clone())
      } else if self.contains(&point) {
        Some(point)
      } else {
        None
      }
    } else {
      None
    }
  }
}

#[derive(Debug, Clone)]
struct Face {
  indices: [i8; 3],
  verts: [Vec3; 3],
}

impl Face {
  // Returns plane intersection points
  #[allow(dead_code)]
  fn plane_intersection(&self, plane: &Plane) -> Option<[Vec3; 2]> {
    let ab = Edge {
      a: self.verts[0],
      b: self.verts[1],
    };
    let ab_intercept = ab.plane_intersection(plane);
    let bc = Edge {
      a: self.verts[1],
      b: self.verts[2],
    };
    let bc_intercept = bc.plane_intersection(plane);
    let ca = Edge {
      a: self.verts[2],
      b: self.verts[0],
    };
    let ca_intercept = ca.plane_intersection(plane);
    match (ab_intercept, bc_intercept, ca_intercept) {
      (Some(a), Some(b), None) => Some([a, b]),
      (None, Some(a), Some(b)) => Some([a, b]),
      (Some(a), None, Some(b)) => Some([a, b]),
      (None, None, None) => None,
      (Some(a), Some(b), Some(c)) => {
        // One of these is a duplicate
        if (c - a).norm() < EPSILON {
          Some([a, b])
        } else if (c - b).norm() < EPSILON {
          Some([a, b])
        } else if (b - a).norm() < EPSILON {
          Some([a, c])
        } else {
          unreachable!()
        }
      }
      _ => unreachable!(),
    }
  }
}

#[derive(Debug, Clone)]
struct NGon {
  verts: Vec<Vec3>,
}

impl NGon {
  fn plane_split(&self, plane: &Plane) -> Vec<Self> {
    if self.verts.len() > 2 {
      // Get edges
      let mut edges: Vec<Edge> = vec![];
      for i in 0..(self.verts.len() - 1) {
        edges.push(Edge {
          a: self.verts[i],
          b: self.verts[i + 1],
        });
      }
      edges.push(Edge {
        a: self.verts[self.verts.len() - 1],
        b: self.verts[0],
      });

      // Find unique edge splits
      let mut intersections: Vec<Option<Vec3>> = vec![];
      for edge in edges.iter() {
        let next_split = edge.plane_intersection(plane);
        if let Some(next_split) = next_split {
          // Is it unqiue?
          let unique = intersections
            .iter()
            .filter(|i| i.is_some())
            .map(|i| i.unwrap())
            .all(|i| (i - next_split).norm() > EPSILON);
          if unique {
            intersections.push(Some(next_split));
          } else {
            intersections.push(None);
          }
        } else {
          intersections.push(None);
        }
      }

      // Are we splitting?
      let first_intersection = intersections
        .iter()
        .filter(|i| i.is_some())
        .map(|i| i.unwrap())
        .nth(0);
      if let Some(first_intersection) = first_intersection {
        // Find any other unique split point
        let next_intersection = intersections
          .iter()
          .filter(|i| {
            if let Some(i) = i {
              (i - first_intersection).norm() > EPSILON
            } else {
              false
            }
          })
          .map(|i| i.unwrap())
          .nth(0);

        if let Some(next_intersection) = next_intersection {
          // Work out the order of the intersection splits
          // Once sorted then we can treat them as pairs of
          // 0->1, 2->3, 4->5
          let mut ordered: Vec<(f32, usize)> = Default::default();
          for (idx, intersection) in intersections.iter().enumerate() {
            if let Some(intersection) = intersection {
              let distance = (intersection - first_intersection).norm()
                / (next_intersection - first_intersection).norm();
              ordered.push((distance, idx));
            }
          }
          ordered.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

          // Now we add the maps so we can look up the next
          // edge the cut takes us too
          let mut next_cut: Vec<Option<usize>> = vec![None; edges.len()];
          for i in (0..ordered.len()).step_by(2) {
            let a = intersections[ordered[i].1].unwrap();
            let b = intersections[ordered[i + 1].1].unwrap();
            let mut duplicate = false;
            for edge in edges.iter() {
              // Check for cuts that duplicate current edges
              if ((edge.a - a).norm() < EPSILON || (edge.b - a).norm() < EPSILON)
                && ((edge.a - b).norm() < EPSILON || (edge.b - b).norm() < EPSILON)
              {
                duplicate = true;
                break;
              }
            }
            if duplicate {
              intersections[ordered[i].1] = None;
              intersections[ordered[i + 1].1] = None;
            } else {
              next_cut[ordered[i].1] = Some(ordered[i + 1].1);
              next_cut[ordered[i + 1].1] = Some(ordered[i].1);
            }
          }

          // We can finally build the NGONs
          let mut results = vec![];
          let mut starts = vec![0];
          let mut visited: HashSet<usize> = Default::default();
          while starts.len() > 0 {
            let mut verts = vec![];
            let start_i = starts.pop().unwrap();
            let mut i = start_i;
            while i < edges.len() {
              visited.insert(i);
              verts.push(edges[i].a.clone());
              // Do we edge cut?
              if let Some(intersection) = intersections[i] {
                if !visited.contains(&(i + 1)) {
                  starts.push(i + 1);
                }
                verts.push(intersection.clone());
                let next_edge = next_cut[i].unwrap();
                verts.push(intersections[next_edge].unwrap());
                i = next_edge + 1;
              } else {
                i += 1;
              }

              if i == start_i {
                break;
              }
            }
            let mut filterd_verts: Vec<Vec3> = vec![];
            for vert in verts.into_iter() {
              if let Some(last) = filterd_verts.last() {
                if let Some(first) = filterd_verts.first() {
                  if (vert - last).norm() > EPSILON && (vert - first).norm() > EPSILON {
                    filterd_verts.push(vert);
                  }
                }
              } else {
                filterd_verts.push(vert);
              }
            }
            let verts = filterd_verts;
            if verts.len() > 2 {
              results.push(NGon { verts });
            } else if verts.len() > 0 {
              // Should have no cases where verts.len() == 1 || 2
              println!("no verts B? {:?}", verts);
              println!("verts {:?}", self.verts);
              println!("intersection {:?}", intersections);
              println!("results {:?}", results);
              unreachable!();
            }
          }
          return results;
        }
      }
    }

    return vec![NGon {
      verts: self.verts.clone(),
    }];
  }

  // Read up on Polygon triangulation
  // Here we are using fan triangulation which only works on convex polygons
  // However if we start with a triangle and only plane split all
  // ngons should be convex
  fn fan_triangulation(&self) -> Vec<usize> {
    let mut results = vec![];
    let a = 0;
    for b in 1..(self.verts.len() - 1) {
      let c = b + 1;
      results.push(a);
      results.push(b);
      results.push(c);
    }
    return results;
  }
}

struct NewEdge {
  a: usize,
  b: usize,
  weight: f32,
}

impl PartialEq for NewEdge {
  fn eq(&self, other: &Self) -> bool {
    self.a == other.a && self.b == other.b && (self.weight - other.weight).abs() < EPSILON
  }
}
impl Eq for NewEdge {}

struct NewBary {
  a: usize,
  b: usize,
  c: usize,
  weight_a: f32,
  weight_b: f32,
}

impl PartialEq for NewBary {
  fn eq(&self, other: &Self) -> bool {
    self.a == other.a
      && self.b == other.b
      && self.c == other.c
      && (self.weight_a - other.weight_a).abs() < EPSILON
      && (self.weight_b - other.weight_b).abs() < EPSILON
  }
}
impl Eq for NewBary {}

#[derive(Clone)]
enum Index {
  Corner(usize),
  Edge(usize),
  Bary(usize),
}

fn main() {
  let corners: [Vec3; 8] = [
    [0., 0., 0.].into(),
    [1., 0., 0.].into(),
    [1., 1., 0.].into(),
    [0., 1., 0.].into(),
    [0., 0., 1.].into(),
    [1., 0., 1.].into(),
    [1., 1., 1.].into(),
    [0., 1., 1.].into(),
  ];
  let edges: [Vec3; 12] = [
    (corners[0] + corners[1]) / 2.,
    (corners[1] + corners[2]) / 2.,
    (corners[2] + corners[3]) / 2.,
    (corners[3] + corners[0]) / 2.,
    (corners[4] + corners[5]) / 2.,
    (corners[5] + corners[6]) / 2.,
    (corners[6] + corners[7]) / 2.,
    (corners[7] + corners[4]) / 2.,
    (corners[0] + corners[4]) / 2.,
    (corners[1] + corners[5]) / 2.,
    (corners[2] + corners[6]) / 2.,
    (corners[3] + corners[7]) / 2.,
  ];

  let values: [[bool; 8]; 256] = (0..256)
    .into_iter()
    .map(|cube_index| {
      [
        (cube_index & 1) == 0,
        (cube_index & 2) == 0,
        (cube_index & 4) == 0,
        (cube_index & 8) == 0,
        (cube_index & 16) == 0,
        (cube_index & 32) == 0,
        (cube_index & 64) == 0,
        (cube_index & 128) == 0,
      ]
    })
    .collect::<Vec<[bool; 8]>>()
    .try_into()
    .unwrap();

  // Standard x, y, z planes
  let mut planes = vec![
    Plane {
      origin: [0.5, 0.5, 0.5].into(),
      normal: [1., 0., 0.].into(),
    },
    Plane {
      origin: [0.5, 0.5, 0.5].into(),
      normal: [0., 1., 0.].into(),
    },
    Plane {
      origin: [0.5, 0.5, 0.5].into(),
      normal: [0., 0., 1.].into(),
    },
  ];

  // These planes help form definitive lines on
  // the ambigous cases
  let permutation = [[1, 2], [0, 2], [0, 1]];
  for &u in [0.5, -0.5].iter() {
    for &v in [0.5, -0.5].iter() {
      for axis in permutation.iter() {
        let mut normal = [0.; 3];
        normal[axis[0]] = u;
        normal[axis[1]] = v;

        planes.push(Plane {
          origin: [0.5, 0.5, 0.5].into(),
          normal: normal.into(),
        });
      }
    }
  }

  let mut new_edges = vec![];
  let mut new_barys = vec![];

  let mut new_triangle_table_unmapped = vec![];
  let mut corner_tables = vec![];
  let mut uv_tables = vec![];

  // let test_cube = 0 | 1 | 2 | 4 | 8;
  // println!("Orig: {:?}", TRIANGLE_TABLE[test_cube]);
  // println!(
  //   "Orig Verts: {:?}",
  //   TRIANGLE_TABLE[test_cube]
  //     .iter()
  //     .filter(|&&i| i >= 0)
  //     .map(|&i| {
  //       let e: [f32; 3] = edges[i as usize].into();
  //       e
  //     })
  //     .collect::<Vec<_>>()
  // );
  // for faces in [TRIANGLE_TABLE[test_cube]].iter() {
  for (cube_index, faces) in TRIANGLE_TABLE.iter().enumerate() {
    let mut new_verts = vec![];
    let mut corner_table = vec![];
    let mut uv_table = vec![];
    for face in &faces.into_iter().chunks(3) {
      let face = face.map(|i| *i).collect::<Vec<i8>>();
      if face.iter().any(|&i| i < 0) {
        break;
      }
      let indices: [i8; 3] = face.try_into().unwrap();
      let verts: Vec<Vec3> = indices
        .iter()
        .map(|&i| edges[i as usize])
        .collect::<Vec<Vec3>>();
      let face = NGon { verts };

      // Split the face on every plane
      let mut ngons = vec![face.clone()];
      for plane in planes.iter() {
        let mut new_ngons = vec![];
        for ngon in ngons.iter() {
          for plane_gon in ngon.plane_split(plane).into_iter() {
            new_ngons.push(plane_gon);
          }
        }
        ngons = new_ngons;
      }

      let permutation = [[1, 2], [0, 2], [0, 1]];

      if ngons.len() == 1 {
        // Just copy over this one
        new_verts.push(Index::Corner(indices[0] as usize));
        new_verts.push(Index::Corner(indices[1] as usize));
        new_verts.push(Index::Corner(indices[2] as usize));

        // Face normal
        let ab = face.verts[1] - face.verts[0];
        let ac = face.verts[2] - face.verts[0];
        let normal = ac.normalize().cross(&ab.normalize());

        // Face center
        let cen = (face.verts[0] + face.verts[1] + face.verts[2]) / 3.;
        let cen_epsi = cen + normal * EPSILON;

        // Which corner are we closest too
        let dist: Vec<_> = corners.iter().map(|p| (cen_epsi - p).norm()).collect();
        let mut i = 0;
        let mut v = None;
        for (j, &value) in dist.iter().enumerate() {
          // Filter out the corners that are not
          // inside the isosurface
          if values[cube_index][j] {
            if let Some(va) = v {
              if value < va {
                i = j;
                v = Some(value);
              }
            } else {
              i = j;
              v = Some(value);
            }
          }
        }
        assert!(v.is_some());
        corner_table.push(i as isize);
        corner_table.push(i as isize);
        corner_table.push(i as isize);

        // UVs
        // Is the normal pointing along x, y, or z
        // We use that to decide how to map the uvs
        // dot product gives the cosine of the angle
        // between.
        let x_axis: Vec3 = [1., 0., 0.].into();
        let y_axis: Vec3 = [0., 1., 0.].into();
        let z_axis: Vec3 = [0., 0., 1.].into();
        let cos = [
          normal.dot(&x_axis),
          normal.dot(&y_axis),
          normal.dot(&z_axis),
        ];

        let octant_origin: Vec3 = [
          (cen_epsi[0] * 2.).floor() / 2.,
          (cen_epsi[1] * 2.).floor() / 2.,
          (cen_epsi[2] * 2.).floor() / 2.,
        ]
        .into();

        // Nearest axis alignment is found here
        // We take abs and find the maximum
        let mut i = 0;
        for (j, &value) in cos.iter().enumerate() {
          if value > cos[i].abs() {
            i = j;
          }
        }
        let max_cos = cos[i];
        let uv_invert = max_cos < 0.;
        let uv_axis = permutation[i];
        let mut uvs: Vec<_> = face
          .verts
          .iter()
          .map(|v| {
            [
              match uv_invert {
                false => v[uv_axis[0]] - octant_origin[uv_axis[0]],
                true => 1. - (v[uv_axis[0]] - octant_origin[uv_axis[0]]),
              },
              match uv_invert {
                false => v[uv_axis[1]] - octant_origin[uv_axis[1]],
                true => 1. - (v[uv_axis[1]] - octant_origin[uv_axis[1]]),
              },
            ]
          })
          .collect();
        if uvs[0][0].is_nan()
          || uvs[0][1].is_nan()
          || uvs[1][0].is_nan()
          || uvs[1][1].is_nan()
          || uvs[2][0].is_nan()
          || uvs[2][1].is_nan()
        {
          println!("UVs contain NAN");
          println!("uvs: {:?}", [&uvs[0], &uvs[1], &uvs[2]]);
          println!("verts: {:?}", face.verts);
          println!("ab: {:?}", ab);
          println!("ac: {:?}", ac);
          println!("normal: {:?}", normal);
          unreachable!();
        } else if uvs[0][0] < -EPSILON
          || uvs[0][0] > 1. + EPSILON
          || uvs[0][1] < -EPSILON
          || uvs[0][1] > 1. + EPSILON
          || uvs[1][0] < -EPSILON
          || uvs[1][0] > 1. + EPSILON
          || uvs[1][1] < -EPSILON
          || uvs[1][1] > 1. + EPSILON
          || uvs[2][0] < -EPSILON
          || uvs[2][0] > 1. + EPSILON
          || uvs[2][1] < -EPSILON
          || uvs[2][1] > 1. + EPSILON
        {
          println!("UVs not in range");
          println!("uvs: {:?}", [&uvs[0], &uvs[1], &uvs[2]]);
          println!("verts: {:?}", face.verts);
          println!("ab: {:?}", ab);
          println!("ac: {:?}", ac);
          println!("normal: {:?}", normal);
          unreachable!();
        }

        uvs[0][0] = uvs[0][0].clamp(0., 1.);
        uvs[0][1] = uvs[0][1].clamp(0., 1.);
        uvs[1][0] = uvs[1][0].clamp(0., 1.);
        uvs[1][1] = uvs[1][1].clamp(0., 1.);
        uvs[2][0] = uvs[2][0].clamp(0., 1.);
        uvs[2][1] = uvs[2][1].clamp(0., 1.);

        uv_table.push(uvs[0].clone());
        uv_table.push(uvs[1].clone());
        uv_table.push(uvs[2].clone());
        continue;
      }

      // Build up the polygons from the ngons
      // But use a unique vertex map
      let mut unique_verts: Vec<Vec3> = vec![];
      let mut polys = vec![];
      for ngon in ngons.iter() {
        let ngon_polygons = ngon.fan_triangulation();
        for i in ngon_polygons {
          let vert = ngon.verts[i];
          let idx = unique_verts
            .iter()
            .position(|v| (v - vert).norm() < EPSILON);
          if let Some(idx) = idx {
            polys.push(idx);
          } else {
            polys.push(unique_verts.len());
            unique_verts.push(vert.clone());
          }
        }
      }

      // Now to build up the global unique verts map
      // We first work out the barycentric coordinates
      // for all unique verts
      let mut sorted_indices = indices.to_vec();
      sorted_indices.sort();
      let sorted_verts: Vec<Vec3> = sorted_indices
        .iter()
        .map(|&i| edges[i as usize])
        .collect::<Vec<Vec3>>();

      let barycentrics: Vec<_> = unique_verts
        .iter()
        .map(|v| xyz_to_barycentric(v, &sorted_verts[0], &sorted_verts[1], &sorted_verts[2]))
        .collect();

      // Maps local index to gloabl index
      let mut index_map = vec![];

      for barycentric in barycentrics.iter() {
        if let Some(idx) = barycentric.iter().position(|i| (i - 1.).abs() < EPSILON) {
          // This is at a corner point
          index_map.push(Index::Corner(sorted_indices[idx].try_into().unwrap()));
        } else if let Some(idx) = barycentric.iter().position(|i| (i - 0.).abs() < EPSILON) {
          // This is an edge vertex
          let j = permutation[idx][0];
          let k = permutation[idx][1];
          let a = sorted_indices[j];
          let b = sorted_indices[k];
          let weight = barycentric[j];
          let new_edge = NewEdge {
            a: a.try_into().unwrap(),
            b: b.try_into().unwrap(),
            weight,
          };
          let idx = new_edges.iter().position(|e| e == &new_edge);
          if let Some(idx) = idx {
            index_map.push(Index::Edge(idx));
          } else {
            index_map.push(Index::Edge(new_edges.len()));
            new_edges.push(new_edge);
          }
        } else {
          // This is a barycentric vertex
          let a = sorted_indices[0];
          let b = sorted_indices[1];
          let c = sorted_indices[2];
          let weight_a = barycentric[0];
          let weight_b = barycentric[1];
          let new_bary = NewBary {
            a: a.try_into().unwrap(),
            b: b.try_into().unwrap(),
            c: c.try_into().unwrap(),
            weight_a,
            weight_b,
          };
          let idx = new_barys.iter().position(|b| b == &new_bary);
          if let Some(idx) = idx {
            index_map.push(Index::Bary(idx));
          } else {
            index_map.push(Index::Bary(new_barys.len()));
            new_barys.push(new_bary);
          }
        }
      }

      for i in (0..polys.len()).step_by(3) {
        let l_a = polys[i];
        let l_b = polys[i + 1];
        let l_c = polys[i + 2];
        let a = index_map[l_a].clone();
        let b = index_map[l_b].clone();
        let c = index_map[l_c].clone();
        new_verts.push(a);
        new_verts.push(b);
        new_verts.push(c);

        let v1 = unique_verts[l_a];
        let v2 = unique_verts[l_b];
        let v3 = unique_verts[l_c];

        // Face normal
        let ab = v2 - v1;
        let ac = v3 - v1;
        let normal = ac.normalize().cross(&ab.normalize());

        // Face center
        let cen = (v1 + v2 + v3) / 3.;
        let cen_epsi = cen + normal * EPSILON;

        // Which corner are we closest too
        let dist: Vec<_> = corners.iter().map(|p| (cen_epsi - p).norm()).collect();
        let mut i = 0;
        let mut v = None;
        for (j, &value) in dist.iter().enumerate() {
          // Filter out the corners that are not
          // inside the isosurface
          if values[cube_index][j] {
            if let Some(va) = v {
              if value < va {
                i = j;
                v = Some(value);
              }
            } else {
              i = j;
              v = Some(value);
            }
          }
        }

        assert!(v.is_some());
        corner_table.push(i as isize);
        corner_table.push(i as isize);
        corner_table.push(i as isize);

        // UVs
        // Is the normal pointing along x, y, or z
        // We use that to decide how to map the uvs
        // dot product gives the cosine of the angle
        // between.
        let x_axis: Vec3 = [1., 0., 0.].into();
        let y_axis: Vec3 = [0., 1., 0.].into();
        let z_axis: Vec3 = [0., 0., 1.].into();
        let cos = [
          normal.dot(&x_axis),
          normal.dot(&y_axis),
          normal.dot(&z_axis),
        ];

        let octant_origin: Vec3 = [
          (cen_epsi[0] * 2.).floor() / 2.,
          (cen_epsi[1] * 2.).floor() / 2.,
          (cen_epsi[2] * 2.).floor() / 2.,
        ]
        .into();

        // Nearest axis alignment is this one!
        // We take abs and find the maximum
        let mut i = 0;
        for (j, &value) in cos.iter().enumerate() {
          if value > cos[i].abs() {
            i = j;
          }
        }
        let max_cos = cos[i];
        let uv_invert = max_cos < 0.;
        let uv_axis = permutation[i];
        let mut uvs: Vec<_> = [v1, v2, v3]
          .iter()
          .map(|v| {
            [
              match uv_invert {
                false => v[uv_axis[0]] - octant_origin[uv_axis[0]],
                true => 1. - (v[uv_axis[0]] - octant_origin[uv_axis[0]]),
              },
              match uv_invert {
                false => v[uv_axis[1]] - octant_origin[uv_axis[1]],
                true => 1. - (v[uv_axis[1]] - octant_origin[uv_axis[1]]),
              },
            ]
          })
          .collect();
        if uvs[0][0].is_nan()
          || uvs[0][1].is_nan()
          || uvs[1][0].is_nan()
          || uvs[1][1].is_nan()
          || uvs[2][0].is_nan()
          || uvs[2][1].is_nan()
        {
          println!("UVs contain NAN");
          println!("uvs: {:?}", [&uvs[0], &uvs[1], &uvs[2]]);
          println!("verts: {:?}", [v1, v2, v3]);
          println!("ab: {:?}", ab);
          println!("ac: {:?}", ac);
          println!("normal: {:?}", normal);
          unreachable!();
        } else if uvs[0][0] < -EPSILON
          || uvs[0][0] > 1. + EPSILON
          || uvs[0][1] < -EPSILON
          || uvs[0][1] > 1. + EPSILON
          || uvs[1][0] < -EPSILON
          || uvs[1][0] > 1. + EPSILON
          || uvs[1][1] < -EPSILON
          || uvs[1][1] > 1. + EPSILON
          || uvs[2][0] < -EPSILON
          || uvs[2][0] > 1. + EPSILON
          || uvs[2][1] < -EPSILON
          || uvs[2][1] > 1. + EPSILON
        {
          println!("UVs not in range");
          println!("uvs: {:?}", [&uvs[0], &uvs[1], &uvs[2]]);
          println!("verts: {:?}", [v1, v2, v3]);
          println!("ab: {:?}", ab);
          println!("ac: {:?}", ac);
          println!("normal: {:?}", normal);
          unreachable!();
        }

        uvs[0][0] = uvs[0][0].clamp(0., 1.);
        uvs[0][1] = uvs[0][1].clamp(0., 1.);
        uvs[1][0] = uvs[1][0].clamp(0., 1.);
        uvs[1][1] = uvs[1][1].clamp(0., 1.);
        uvs[2][0] = uvs[2][0].clamp(0., 1.);
        uvs[2][1] = uvs[2][1].clamp(0., 1.);

        uv_table.push(uvs[0].clone());
        uv_table.push(uvs[1].clone());
        uv_table.push(uvs[2].clone());
      }
    }
    new_triangle_table_unmapped.push(new_verts);
    corner_tables.push(corner_table);
    uv_tables.push(uv_table);
  }

  let longest = new_triangle_table_unmapped
    .iter()
    .map(|c| c.len())
    .max()
    .unwrap()
    + 1;
  let ordinary_len = edges.len();
  let special_edges_len = new_edges.len();
  let special_bary_len = new_barys.len();

  let mut new_triangle_table: Vec<_> = new_triangle_table_unmapped
    .iter()
    .map(|t| {
      t.iter()
        .map(|i| match i {
          Index::Corner(i) => *i as isize,
          Index::Edge(i) => (i + ordinary_len) as isize,
          Index::Bary(i) => (i + ordinary_len + special_edges_len) as isize,
        })
        .collect::<Vec<isize>>()
    })
    .collect::<Vec<Vec<isize>>>();
  new_triangle_table
    .iter_mut()
    .for_each(|c| c.append(&mut vec![-1; longest - c.len()]));

  println!("pub const EDGE_TABLE: [u32; 256] = {:?};", EDGE_TABLE);
  println!(
    "pub const TRIANGLE_TABLE: [[isize; {}]; 256] = {:?};",
    longest, new_triangle_table
  );
  if special_edges_len > 0 || special_bary_len > 0 {
    println!("pub const ORDINARY_EDGE_LEN: usize = {:?};", ordinary_len);
  }

  if special_edges_len > 0 {
    println!(
      "pub const SPECIAL_EDGE_LEN: usize = {:?};",
      special_edges_len
    );
    println!(
      "pub const SPECIAL_EDGES: [(usize,usize,f32); {}] = {:?};",
      special_edges_len,
      new_edges
        .iter()
        .map(|e| (e.a, e.b, e.weight))
        .collect::<Vec<(usize, usize, f32)>>()
    );
  }

  if special_bary_len > 0 {
    println!(
      "// pub const SPECIAL_BARY_LEN: usize = {:?};",
      special_bary_len
    );
    println!(
      "pub const SPECIAL_BARYS: [(usize,usize, usize,f32, f32); {}] = {:?};",
      special_bary_len,
      new_barys
        .iter()
        .map(|b| (b.a, b.b, b.c, b.weight_a, b.weight_b))
        .collect::<Vec<(usize, usize, usize, f32, f32)>>()
    );
  }

  corner_tables
    .iter_mut()
    .for_each(|c| c.append(&mut vec![-1; longest - c.len()]));
  uv_tables
    .iter_mut()
    .for_each(|c| c.append(&mut vec![[-1., -1.]; longest - c.len()]));
  println!(
    "pub const UV_TABLE: [[[f32;2]; {}]; 256] = {:?};",
    longest, uv_tables
  );
  println!(
    "pub const CORNER_TABLE: [[i8; {}]; 256] = {:?};",
    longest, corner_tables
  );
}
