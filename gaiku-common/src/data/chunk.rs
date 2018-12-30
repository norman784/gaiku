#[derive(Debug)]
pub struct Chunk {
    size: usize,
    is_2d: bool,
    values: Vec<f32>,
}

impl Chunk {
    pub fn new(size: usize, is_2d: bool) -> Self {
        let length  = {
            if is_2d {
                size * size
            } else {
                size * size * size
            }
        };

        Chunk {
            size,
            is_2d,
            values: vec![0.0; length]
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn add(&mut self, (x, y, z): (usize, usize, usize), value: f32) {
        self.values.insert(self.index((x, y, z)),  value);
    }

    pub fn get(&self, (x, y, z): (i32, i32, i32)) -> f32 {
        if x < 0 || y < 0 || z < 0 {
            0.0
        } else {
            self.values[self.index((x as usize, y as usize, z as usize))]
        }
    }

    pub fn is_empty(&self, (x, y, z): (i32, i32, i32)) -> bool {
        self.get((x, y, z)) == 0.0
    }

    fn index(&self, (x, y, z): (usize, usize, usize)) -> usize {
        if self.is_2d {
            x + self.size * y
        } else {
            x + self.size * y + self.size * z
        }
    }
}