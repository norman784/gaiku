use crate::Vector3i;

#[derive(Debug)]
pub struct Chunk {
    position: Vector3i,
    width: usize,
    height: usize,
    depth: usize,
    values: Vec<f32>,
}

impl Chunk {
    pub fn new(position: [i32; 3], width: usize, height: usize, depth: usize) -> Self {
        Chunk {
            position: position.into(),
            width,
            height,
            depth,
            values: vec![0.0; depth * height * width],
        }
    }

    pub fn get_position(&self) -> Vector3i {
        self.position
    }

    pub fn set_position(&mut self, value: Vector3i) {
        self.position = value;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn depth(&self) -> usize {
        self.depth
    }

    pub fn add(&mut self, x: usize, y: usize, z: usize, value: f32) {
        let index = self.index(x, y, z);
        self.values[index] = value;
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> f32 {
        self.values[self.index(x, y, z)]
    }

    pub fn is_air(&self, x: usize, y: usize, z: usize) -> bool {
        if x >= self.width || y >= self.height || z >= self.depth {
            true
        } else {
            self.values[self.index(x, y, z)] == 0.0
        }
    }

    fn index(&self, x: usize, y: usize, z: usize) -> usize {
        x + y * self.width + z * self.width * self.height
    }
}