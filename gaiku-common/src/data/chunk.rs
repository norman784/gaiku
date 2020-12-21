use mint::{Vector3, Vector4};

// TODO: Get inspiration on multiarray crate (https://github.com/sellibitze/multiarray) to make chunk 2d and 3d friendly

#[derive(Debug, Clone, TypedBuilder, Getters, Setters)]
pub struct Chunk {
    #[get = "pub"]
    #[set = "pub"]
    colors: Vec<Vector4<u8>>,
    #[get = "pub"]
    #[set = "pub"]
    position: Vector3<f32>,
    #[get = "pub"]
    width: usize,
    #[get = "pub"]
    height: usize,
    #[get = "pub"]
    depth: usize,
    #[get = "pub"]
    #[set = "pub"]
    values: Vec<u8>,
}

impl Chunk {
    pub fn new(position: [f32; 3], width: usize, height: usize, depth: usize) -> Self {
        Chunk {
            colors: vec![[0, 0, 0, 0].into(); depth * height * width],
            position: position.into(),
            width,
            height,
            depth,
            values: vec![0; depth * height * width],
        }
    }

    // pub fn clone(&self) -> Self {
    //     Chunk {
    //         colors: vec![],
    //         position: self.position.clone(),
    //         width: self.width,
    //         height: self.height,
    //         depth: self.depth,
    //         values: self.values.clone(),
    //     }
    // }

    pub fn is_air(&self, x: usize, y: usize, z: usize) -> bool {
        if x >= self.width || y >= self.height || z >= self.depth {
            true
        } else {
            self.values[self.index(x, y, z)] == 0
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> u8 {
        self.values[self.index(x, y, z)]
    }

    pub fn get_color(&self, x: usize, y: usize, z: usize) -> Option<Vector4<u8>> {
        let index = self.index(x, y, z);
        if let Some(color) = self.colors.get(index) {
            Some(*color)
        } else {
            None
        }
    }

    fn index(&self, x: usize, y: usize, z: usize) -> usize {
        get_index_from(x, y, z, self.width, self.height, self.depth)
    }

    pub fn set(&mut self, x: usize, y: usize, z: usize, value: u8) {
        if value != 0 {
            self.set_with_color(x, y, z, value, [0, 0, 0, 1].into());
        } else {
            self.set_with_color(x, y, z, value, [0, 0, 0, 0].into());
        }
    }

    pub fn set_with_color(&mut self, x: usize, y: usize, z: usize, value: u8, color: Vector4<u8>) {
        let index = self.index(x, y, z);
        self.values[index] = value;
        self.colors[index] = color;
    }

    // TODO: This will add  the neighbor data at the border of the chunk, so we can calculate correctly  the normals, heights, etc without need to worry to query each time to get that data
    pub fn update_neighbor_data(&self, _neighbor: &Chunk) {
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
