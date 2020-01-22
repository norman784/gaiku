use mint::Vector3;

// TODO: Get inspiration on multiarray crate (https://github.com/sellibitze/multiarray) to make chunk 2d and 3d friendly

#[derive(Debug, Clone, Builder, Getters, Setters)]
pub struct Chunk {
    #[get = "pub"] #[set = "pub"]
    position: Vector3<f64>,
    #[get = "pub"] #[set = "pub"]
    width: usize,
    #[get = "pub"] #[set = "pub"]
    height: usize,
    #[get = "pub"] #[set = "pub"]
    depth: usize,
    #[get = "pub"] #[set = "pub"]
    values: Vec<f32>,
}

impl Chunk {
    pub fn new(position: [f64; 3], width: usize, height: usize, depth: usize) -> Self {
        Chunk {
            position: position.into(),
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

    pub fn get(&self, x: usize, y: usize, z: usize) -> f32 {
        self.values[self.index(x, y, z)]
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, value: f32) {
        let index = self.index(x, y, z);
        self.values[index] = value;
    }

    fn index(&self, x: usize, y: usize, z: usize) -> usize {
        x + y * self.width + z * self.width * self.height
    }

    // TODO: This will add  the neighbor data at the border of the chunk, so we can calculate correctly  the normals, heights, etc without need to worry to query each time to get that data
    pub fn update_neighbor_data(&self, neighbor: &Chunk) {
        unimplemented!();
    }
}
