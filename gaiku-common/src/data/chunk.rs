use mint::{Vector3, Vector4};

pub trait Chunkify {
  fn new(position: [f32; 3], width: usize, height: usize, depth: usize) -> Self;
  fn depth(&self) -> usize;
  fn is_air(&self, x: usize, y: usize, z: usize) -> bool;
  fn get(&self, x: usize, y: usize, z: usize) -> u8;
  fn height(&self) -> usize;
  fn position(&self) -> Vector3<f32>;
  fn set(&mut self, x: usize, y: usize, z: usize, value: u8);
  fn width(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct Chunk {
  colors: Vec<Vector4<u8>>,
  position: Vector3<f32>,
  width: usize,
  height: usize,
  depth: usize,
  values: Vec<u8>,
}

impl Chunk {
  pub fn get_with_color(&self, x: usize, y: usize, z: usize) -> (u8, Vector4<u8>) {
    let color = if let Some(color) = self.colors.get(self.index(x, y, z)) {
      color
    } else {
      &Vector4 {
        x: 0,
        y: 0,
        z: 0,
        w: 0,
      }
    };

    (self.get(x, y, z), color.clone())
  }

  fn index(&self, x: usize, y: usize, z: usize) -> usize {
    get_index_from(x, y, z, self.width, self.height, self.depth)
  }

  pub fn set_with_color(&mut self, x: usize, y: usize, z: usize, value: u8, color: Vector4<u8>) {
    let index = self.index(x, y, z);
    self.colors[index] = color;
    self.values[index] = value;
  }

  // TODO: This will add  the neighbor data at the border of the chunk, so we can calculate correctly  the normals, heights, etc without need to worry to query each time to get that data
  pub fn update_neighbor_data(&self, _neighbor: &Chunk) {
    unimplemented!();
  }
}

impl Chunkify for Chunk {
  fn new(position: [f32; 3], width: usize, height: usize, depth: usize) -> Self {
    Chunk {
      colors: vec![[0, 0, 0, 0].into(); depth * height * width],
      position: position.into(),
      width,
      height,
      depth,
      values: vec![0; depth * height * width],
    }
  }

  fn depth(&self) -> usize {
    self.depth
  }

  fn is_air(&self, x: usize, y: usize, z: usize) -> bool {
    if x >= self.width || y >= self.height || z >= self.depth {
      true
    } else {
      self.values[self.index(x, y, z)] == 0
    }
  }

  fn get(&self, x: usize, y: usize, z: usize) -> u8 {
    self.values[self.index(x, y, z)]
  }
  fn height(&self) -> usize {
    self.height
  }

  fn position(&self) -> Vector3<f32> {
    self.position
  }

  fn set(&mut self, x: usize, y: usize, z: usize, value: u8) {
    let (_, mut color) = self.get_with_color(x, y, z);

    if color.x == 0 && color.y == 0 && color.z == 0 && color.w == 0 {
      color = if value > 0 {
        [255, 255, 255, 255]
      } else {
        [0, 0, 0, 0]
      }
      .into();
    }

    self.set_with_color(x, y, z, value, color);
  }

  fn width(&self) -> usize {
    self.width
  }
}

pub fn get_index_from(
  x: usize,
  y: usize,
  z: usize,
  width: usize,
  height: usize,
  _depth: usize,
) -> usize {
  x + y * width + z * width * height
}
