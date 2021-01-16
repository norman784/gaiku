/// Define a component that contains a position.
pub trait Positionable {
  fn with_position(position: [f32; 3]) -> Self;
  fn position(&self) -> [f32; 3];
}

/// Define a component that contains a 3d size.
pub trait Sizable {
  fn with_size(width: u16, height: u16, depth: u16) -> Self;
  fn width(&self) -> u16;
  fn height(&self) -> u16;
  fn depth(&self) -> u16;
}

/// Define a component that is `Sizable` and `Positionable`, also a initializer
pub trait Boxify: Positionable + Sizable {
  fn new(position: [f32; 3], width: u16, height: u16, depth: u16) -> Self;
}
