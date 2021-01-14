use mint::{Vector3, Vector4};

pub trait Chunkify {
  fn new(position: [f32; 3], width: usize, height: usize, depth: usize) -> Self;
  fn depth(&self) -> usize;
  fn is_air(&self, x: usize, y: usize, z: usize) -> bool;
  fn get(&self, x: usize, y: usize, z: usize) -> u8;
  fn get_color(&self, x: usize, y: usize, z: usize) -> Vector4<u8>;
  fn height(&self) -> usize;
  fn position(&self) -> Vector3<f32>;
  fn width(&self) -> usize;
  fn get_with_color(&self, x: usize, y: usize, z: usize) -> (u8, Vector4<u8>) {
    (self.get(x, y, z), self.get_color(x, y, z))
  }
}

pub trait ChunkifyMut {
  fn set(&mut self, x: usize, y: usize, z: usize, value: u8);
  fn set_color(&mut self, x: usize, y: usize, z: usize, color: Vector4<u8>);
  fn set_with_color(&mut self, x: usize, y: usize, z: usize, value: u8, color: Vector4<u8>) {
    self.set(x, y, z, value);
    self.set_color(x, y, z, color);
  }
}

pub trait ChunkifyNeighboured {
  fn update_neighbor_data<T: Chunkify + ChunkifyNeighboured + Clone>(&self, _neighbor: &T);
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
  fn index(&self, x: usize, y: usize, z: usize) -> usize {
    get_index_from(x, y, z, self.width, self.height, self.depth)
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
  fn get_color(&self, x: usize, y: usize, z: usize) -> Vector4<u8> {
    let color = if let Some(color) = self.colors.get(self.index(x, y, z)) {
      *color
    } else {
      Vector4 {
        x: 0,
        y: 0,
        z: 0,
        w: 0,
      }
    };

    color
  }
  fn height(&self) -> usize {
    self.height
  }

  fn position(&self) -> Vector3<f32> {
    self.position
  }

  fn width(&self) -> usize {
    self.width
  }
}

impl ChunkifyMut for Chunk {
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
    self.set_color(x, y, z, color);
  }

  fn set_color(&mut self, x: usize, y: usize, z: usize, color: Vector4<u8>) {
    let index = self.index(x, y, z);
    self.colors[index] = color;
  }
}

impl ChunkifyNeighboured for Chunk {
  // TODO: This will add  the neighbor data at the border of the chunk, so we can calculate correctly  the normals, heights, etc without need to worry to query each time to get that data
  fn update_neighbor_data<T: Chunkify + ChunkifyNeighboured + Clone>(&self, _neighbor: &T) {
    unimplemented!();
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
