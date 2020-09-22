use bevy::{input::mouse::MouseMotion, prelude::*};

// ==============
// COMPONENTS
// ==============

pub struct MouseDisplacementControl {
  pub keyboard_modifier: Option<KeyCode>,
  pub mouse_modifier: Option<MouseButton>,
  pub speed: f32,
  pub invert_axis: bool,
  pub movement_threshold: f32,
}

impl Default for MouseDisplacementControl {
  fn default() -> Self {
    Self {
      keyboard_modifier: None,
      mouse_modifier: None,
      speed: 3.,
      invert_axis: true,
      movement_threshold: 3.,
    }
  }
}

pub struct KeyboardDisplacementControl {
  pub forward: KeyCode,
  pub backward: KeyCode,
  pub left: KeyCode,
  pub right: KeyCode,
  pub speed: f32,
}

impl Default for KeyboardDisplacementControl {
  fn default() -> Self {
    Self {
      forward: KeyCode::W,
      backward: KeyCode::S,
      left: KeyCode::A,
      right: KeyCode::D,
      speed: 10.,
    }
  }
}

// ==============
// SYSTEMS
// ==============

pub fn keyboard_displacement_system(
  time: Res<Time>,
  keyboard_input: Res<Input<KeyCode>>,
  mut query: Query<(&KeyboardDisplacementControl, &mut Transform)>,
) {
  for (control, mut transform) in &mut query.iter() {
    let mut movement = Vec2::new(
      axis_for_input(&keyboard_input, control.right, control.left),
      axis_for_input(&keyboard_input, control.forward, control.backward),
    );

    if movement != Vec2::zero() {
      movement.normalize();
    }

    movement *= time.delta_seconds * control.speed;
    let fwd = transform.value().z_axis().truncate() * movement.y();
    let right = -transform.value().x_axis().truncate() * movement.x();

    let translation = transform.translation();
    transform.set_translation(translation + Vec3::from(fwd + right));
  }
}

// TODO: Add ability to move the target using the rotation as reference, like blender pan (shift + middle mouse button)
pub fn mouse_displacement_system(
  time: Res<Time>,
  mut event: ResMut<MouseDisplacementEventReader>,
  keyboard_input: Res<Input<KeyCode>>,
  mouse_input: Res<Input<MouseButton>>,
  mouse_motion: Res<Events<MouseMotion>>,
  mut query: Query<(&MouseDisplacementControl, &mut Transform)>,
) {
  let mut delta = Vec2::zero();

  for event in event.motion.iter(&mouse_motion) {
    delta += event.delta;
  }

  for (control, mut transform) in &mut query.iter() {
    if let Some(key) = control.keyboard_modifier {
      if !keyboard_input.pressed(key) {
        return;
      }
    }

    if let Some(button) = control.mouse_modifier {
      if !mouse_input.pressed(button) {
        return;
      }
    }

    let invert_axis = if control.invert_axis { -1. } else { 1. };
    let mut movement = Vec2::new(
      axis_for_value(control.movement_threshold, -delta.x() * invert_axis),
      axis_for_value(control.movement_threshold, delta.y() * invert_axis),
    );

    if movement != Vec2::zero() {
      movement.normalize();
    }

    movement *= time.delta_seconds * control.speed;

    let fwd = transform.value().z_axis().truncate() * movement.y();
    let right = -transform.value().x_axis().truncate() * movement.x();

    let translation = transform.translation();
    transform.set_translation(translation + Vec3::from(fwd + right));
  }
}

// ==============
// HELPERS
// ==============

#[derive(Default)]
pub struct MouseDisplacementEventReader {
  pub button: EventReader<MouseButton>,
  pub motion: EventReader<MouseMotion>,
}

fn axis_for_input(keyboard_input: &Res<Input<KeyCode>>, minus: KeyCode, plus: KeyCode) -> f32 {
  if keyboard_input.pressed(minus) {
    -1.
  } else if keyboard_input.pressed(plus) {
    1.
  } else {
    0.
  }
}

fn axis_for_value(threshold: f32, value: f32) -> f32 {
  if value < -threshold || value > threshold {
    value
  } else {
    0.
  }
}
