use mint::Vector2;

pub const COLS: u16 = 16;
pub const ROWS: u16 = 16;

fn index_to_xy(index: u8) -> (u8, u8) {
  (index % COLS as u8, index / COLS as u8)
}

fn xy_to_uv((x, y): (u8, u8)) -> (f32, f32) {
  (x as f32 / COLS as f32, y as f32 / ROWS as f32)
}

pub struct TextureAtlas2d {
  texture: Texture2d,
}

impl TextureAtlas2d {
  pub fn new(tile_size: u16) -> Self {
    Self::with_texture(Texture2d::new(COLS * tile_size, ROWS * tile_size))
  }

  pub fn with_texture(texture: Texture2d) -> Self {
    Self { texture }
  }

  pub fn get_texture(&self) -> Texture2d {
    self.texture.clone()
  }

  pub fn get_uv(&self, index: u8) -> (Vector2<f32>, Vector2<f32>) {
    let xy = index_to_xy(index);
    let (x, y) = xy_to_uv(xy);
    let x_size = 1.0 / COLS as f32;
    let y_size = 1.0 / ROWS as f32;

    ([x, y].into(), [x + x_size, y + y_size].into())
  }

  pub fn set_at_index(&mut self, index: usize, pixels: Vec<[u8; 4]>) {
    if index + (pixels.len() * 4) < self.texture.data.len() {
      for (p_i, pixel) in pixels.iter().enumerate() {
        for (v_i, value) in pixel.iter().enumerate() {
          self.texture.data[index + p_i + v_i] = *value;
        }
      }
    }
  }
}

pub struct Texture2d {
  width: u16,
  height: u16,
  data: Vec<u8>,
}

impl Texture2d {
  pub fn new(width: u16, height: u16) -> Self {
    Self {
      width,
      height,
      data: vec![0; (width as usize * height as usize) * 4],
    }
  }

  pub fn get_pixel(&self, x: u16, y: u16) -> Option<[u8; 4]> {
    if x < self.width && y < self.height {
      let index = (x + self.width * y) as usize;
      Some([
        self.data[index],
        self.data[index + 1],
        self.data[index + 2],
        self.data[index + 3],
      ])
    } else {
      None
    }
  }

  pub fn height(&self) -> u16 {
    self.height
  }

  pub fn width(&self) -> u16 {
    self.width
  }

  pub fn set_pixel(&mut self, x: u16, y: u16, data: [u8; 4]) {
    if x < self.width && y < self.height {
      let index = (x + self.width * y) as usize;

      self.data[index] = data[0];
      self.data[index + 1] = data[1];
      self.data[index + 2] = data[2];
      self.data[index + 3] = data[3];
    }
  }
}

impl Clone for Texture2d {
  fn clone(&self) -> Self {
    Self {
      width: self.width,
      height: self.height,
      data: self.data.clone(),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  fn get_uv_helper(atlas: &TextureAtlas2d, x: u16, y: u16) -> ([f32; 2], [f32; 2]) {
    let index = x + y * ROWS;
    assert!(index <= 255);
    let uv = atlas.get_uv(index as u8);
    (uv.0.into(), uv.1.into())
  }

  #[test]
  fn test_index_to_xy() {
    assert_eq!((0, 1), index_to_xy(0 + 1 * COLS as u8));
    assert_eq!((1, 0), index_to_xy(1 + 0 * COLS as u8));
    assert_eq!((15, 1), index_to_xy(15 + 1 * COLS as u8));
    assert_eq!((12, 15), index_to_xy(12 + 15 * COLS as u8));
    assert_eq!((15, 15), index_to_xy(15 + 15 * COLS as u8));
  }

  #[test]
  fn test_texture_size() {
    let tile_size = 16;
    let atlas = TextureAtlas2d::new(tile_size);

    assert_eq!(256, atlas.texture.width);
    assert_eq!(256, atlas.texture.height);
  }

  #[test]
  fn test_texture_atlas_index_to_uv() {
    let tile_size = 1;
    let atlas = TextureAtlas2d::new(tile_size);

    let (uv_s, uv_e) = get_uv_helper(&atlas, 0, 0);
    assert_eq!(uv_s, [0.0000, 0.0000]);
    assert_eq!(uv_e, [0.0625, 0.0625]);

    let (uv_s, uv_e) = get_uv_helper(&atlas, 15, 0);
    assert_eq!(uv_s, [0.9375, 0.0000]);
    assert_eq!(uv_e, [1.0000, 0.0625]);

    let (uv_s, uv_e) = get_uv_helper(&atlas, 1, 0);
    assert_eq!(uv_s, [0.0625, 0.0000]);
    assert_eq!(uv_e, [0.1250, 0.0625]);

    let (uv_s, uv_e) = get_uv_helper(&atlas, 15, 15);
    assert_eq!(uv_s, [0.9375, 0.9375]);
    assert_eq!(uv_e, [1.0000, 1.0000]);
  }
}
