use bevy::{
  input::mouse::{MouseMotion, MouseWheel},
  prelude::*,
};

// ==============
// COMPONENTS
// ==============

pub struct RotationControl {
  pub distance: f32,
  pub min_distance: f32,
  pub max_distance: f32,
  pub pitch: f32,
  pub yaw: f32,
  pub rotation_speed: f32,
  pub zoom_speed: f32,
  pub target: Option<Entity>,
}

impl Default for RotationControl {
  fn default() -> Self {
    Self {
      distance: 10.,
      min_distance: 5.,
      max_distance: 200.,
      pitch: 30.0f32.to_radians(),
      yaw: 0.,
      rotation_speed: 1.,
      zoom_speed: 80.,
      target: None,
    }
  }
}

pub struct MouseRotationControl {
  pub keyboard_modifier: Option<KeyCode>,
  pub mouse_modifier: Option<MouseButton>,
  pub invert_axis: bool,
  pub movement_threshold: f32,
}

impl Default for MouseRotationControl {
  fn default() -> Self {
    Self {
      keyboard_modifier: None,
      mouse_modifier: None,
      invert_axis: true,
      movement_threshold: 3.,
    }
  }
}

// ==============
// SYSTEMS
// ==============

pub fn rotate_system(
  mut query: Query<(&mut RotationControl, &mut Transform)>,
  target_query: Query<&mut Transform>,
) {
  for (mut control, mut transform) in &mut query.iter() {
    control.pitch = control
      .pitch
      .max(1f32.to_radians())
      .min(179f32.to_radians());
    control.distance = control
      .distance
      .max(control.min_distance)
      .min(control.max_distance);

    let pos =
      Vec3::new(0., control.pitch.cos(), -control.pitch.sin()).normalize() * control.distance;

    let look = Mat4::face_toward(pos, Vec3::zero(), Vec3::unit_y());
    transform.set_translation(pos);
    transform.set_rotation(look.to_scale_rotation_translation().1);

    if let Some(target) = control.target {
      if let Ok(mut target_transform) = target_query.get_mut::<Transform>(target) {
        target_transform.set_rotation(Quat::from_rotation_y(-control.yaw));
      }
    }
  }
}

pub fn mouse_rotation_system(
  time: Res<Time>,
  mut event: ResMut<RotationControlEventReader>,
  keyboard_input: Res<Input<KeyCode>>,
  mouse_input: Res<Input<MouseButton>>,
  mouse_motion: Res<Events<MouseMotion>>,
  mut query: Query<(&mut RotationControl, &MouseRotationControl)>,
) {
  let mut look = Vec2::zero();

  for event in event.motion.iter(&mouse_motion) {
    look = event.delta;
  }

  for (mut settings, control) in &mut query.iter() {
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

    settings.yaw += look.x() * time.delta_seconds;
    settings.pitch -= look.y() * time.delta_seconds * settings.rotation_speed;
  }
}

pub fn zoom_system(
  time: Res<Time>,
  mut event: ResMut<RotationControlEventReader>,
  mouse_wheel: Res<Events<MouseWheel>>,
  mut query: Query<&mut RotationControl>,
) {
  let mut zoom = 0.;
  for event in event.wheel.iter(&mouse_wheel) {
    zoom = event.y;
  }

  if zoom == 0.1 {
    return;
  }

  for mut settings in &mut query.iter() {
    settings.distance -= zoom * time.delta_seconds * settings.zoom_speed;
  }
}

// ==============
// HELPERS
// ==============

#[derive(Default)]
pub struct RotationControlEventReader {
  pub button: EventReader<MouseButton>,
  pub motion: EventReader<MouseMotion>,
  pub wheel: EventReader<MouseWheel>,
}
