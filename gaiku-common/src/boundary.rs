use mint::Vector3;

const EPSILON: f32 = 1e-5;

#[derive(Clone, Debug)]
pub struct Boundary {
  pub center: Vector3<f32>,
  pub size: Vector3<f32>,
  start: Vector3<f32>,
  end: Vector3<f32>,
}

impl Boundary {
  pub fn new(center: [f32; 3], size: [f32; 3]) -> Self {
    let [cx, cy, cz] = center;
    let [sx, sy, sz] = [size[0] / 2.0, size[1] / 2.0, size[2] / 2.0];
    Self {
      center: center.into(),
      size: size.into(),
      start: [
        cx - sx * (1. - EPSILON),
        cy - sy * (1. - EPSILON),
        cz - sz * (1. - EPSILON),
      ]
      .into(),
      end: [
        cx + sx * (1. + EPSILON),
        cy + sy * (1. + EPSILON),
        cz + sz * (1. + EPSILON),
      ]
      .into(),
    }
  }

  pub fn contains(&self, point: &Vector3<f32>) -> bool {
    self.start.x < point.x
      && self.start.y < point.y
      && self.start.z < point.z
      && self.end.x > point.x
      && self.end.y > point.y
      && self.end.z > point.z
  }

  pub fn intersects(&self, range: &Boundary) -> bool {
    !(range.start.x > self.start.x
      || range.start.y > self.start.y
      || range.start.z > self.start.z
      || range.end.x < self.end.x
      || range.end.y < self.end.y
      || range.end.z < self.end.z)
  }
}
