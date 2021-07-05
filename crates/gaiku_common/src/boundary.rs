use glam::Vec3;

#[derive(Clone, Debug)]
pub struct Boundary {
  pub min: Vec3,
  pub max: Vec3,
}

impl Boundary {
  pub fn new(min: &Vec3, max: &Vec3) -> Self {
    let min: Vec3 = min.min(*max);
    let max: Vec3 = min.max(*max);
    Self { min, max }
  }

  pub fn contains(&self, point: &Vec3) -> bool {
    point.cmpge(self.min).all() && point.cmplt(self.max).all()
  }

  #[allow(clippy::suspicious_operation_groupings)]
  pub fn overlaps(&self, other: &Self) -> bool {
    // Their corner in us
    (other.min.cmpge(self.min).all() && other.min.cmplt(self.max).all()
        || other.max.cmpge(self.min).all() && other.max.cmplt(self.min).all()) ||
        // Our corner in them
        (self.min.cmpge(other.min).all() && self.min.cmplt(other.max).all()
          || self.max.cmpge(other.min).all() && self.max.cmplt(other.max).all())
  }
}
