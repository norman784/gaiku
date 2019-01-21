use acacia::Position;
use nalgebra::Point3;

// TODO: Get inspiration on multiarray crate (https://github.com/sellibitze/multiarray) to make chunk 2d and 3d friendly

#[derive(Debug)]
pub struct Chunk {
    position: Point3<f32>,
    width: usize,
    height: usize,
    depth: usize,
    values: Vec<f32>,
}

impl Chunk {
    pub fn new(position: [f32; 3], width: usize, height: usize, depth: usize) -> Self {
        Chunk {
            position: Point3::new(position[0], position[1], position[2]),
            width,
            height,
            depth,
            values: vec![0.0; depth * height * width],
        }
    }

    pub fn clone(&self) -> Self {
        Chunk {
            position: self.position.clone(),
            width: self.width,
            height: self.height,
            depth: self.depth,
            values: self.values.clone(),
        }
    }

    pub fn is_air(&self, x: usize, y: usize, z: usize) -> bool {
        if x >= self.width || y >= self.height || z >= self.depth {
            true
        } else {
            self.values[self.index(x, y, z)] == 0.0
        }
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

    pub fn get(&self, x: usize, y: usize, z: usize) -> f32 {
        self.values[self.index(x, y, z)]
    }

    pub fn get_position(&self) -> Point3<f32> {
        self.position
    }

    pub fn get_values(&self) -> Vec<f32> {
        self.values.clone()
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, value: f32) {
        let index = self.index(x, y, z);
        self.values[index] = value;
    }

    pub fn set_position(&mut self, value: Point3<f32>) {
        self.position = value;
    }

    pub fn set_values(&mut self, values: Vec<f32>) {
        self.values = values;
    }

    fn index(&self, x: usize, y: usize, z: usize) -> usize {
        x + y * self.width + z * self.width * self.height
    }
}

impl Position for Chunk {
    type Point = Point3<f32>;

    fn position(&self) -> Self::Point {
        self.position
    }
}
