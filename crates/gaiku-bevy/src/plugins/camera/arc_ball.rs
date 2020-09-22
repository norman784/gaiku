use crate::plugins::control::{
  displacement::{
    mouse_displacement_system, MouseDisplacementControl, MouseDisplacementEventReader,
  },
  rotation::{
    mouse_rotation_system, rotate_system, zoom_system, MouseRotationControl, RotationControl,
    RotationControlEventReader,
  },
};
use bevy::{
  prelude::*,
  render::{
    camera::{Camera, PerspectiveProjection, VisibleEntities},
    render_graph::base,
  },
};

// ==============
// ARCHETYPES
// ==============

#[derive(Bundle)]
pub struct ArcBallCameraComponents {
  pub control: ArcBallCameraControl,
  pub camera: Camera,
  pub perspective_projection: PerspectiveProjection,
  pub visible_entities: VisibleEntities,
  pub transform: Transform,
  pub translation: Translation,
  pub rotation: Rotation,
  pub scale: Scale,
}

impl Default for ArcBallCameraComponents {
  fn default() -> Self {
    Self {
      control: ArcBallCameraControl {
        distance: 20.,
        pitch: 30.0f32.to_radians(),
        yaw: 0.,
        target: None,
      },
      camera: Camera {
        name: Some(base::camera::CAMERA3D.to_string()),
        ..Default::default()
      },
      perspective_projection: Default::default(),
      visible_entities: Default::default(),
      transform: Default::default(),
      translation: Translation::new(0., 5., 10.),
      rotation: Rotation::from_rotation_x(-0.4),
      scale: Default::default(),
    }
  }
}

// ==============
// COMPONENTS
// ==============

pub struct ArcBallCameraTarget;

#[derive(Default)]
pub struct ArcBallCameraControl {
  pub distance: f32,
  pub pitch: f32,
  pub yaw: f32,
  pub target: Option<Entity>,
}

// ==============
// SYSTEMS
// ==============

fn face_towards_system(mut query: Query<(&ArcBallCameraControl, &mut Rotation)>) {
  for (control, mut rotation) in &mut query.iter() {
    let cam_pos =
      Vec3::new(0., control.pitch.cos(), -control.pitch.sin()).normalize() * control.distance;
    let look = Mat4::face_toward(cam_pos, Vec3::zero(), Vec3::unit_y());
    rotation.0 = look.to_scale_rotation_translation().1;
  }
}

fn follow_system(
  mut query: Query<(&ArcBallCameraControl, &Rotation, &mut Translation)>,
  target_query: Query<&Translation>,
) {
  for (control, rotation, mut translation) in &mut query.iter() {
    if let Some(target) = control.target {
      if let Ok(target_transform) = target_query.get::<Translation>(target) {
        let position = target_transform.0 - (rotation.0 * -Vec3::unit_z() * control.distance);
        translation.set_x(position.x());
        translation.set_y(position.y());
        translation.set_z(position.z());
      }
    }
  }
}

// ==============
// PLUGINS
// ==============

pub struct ArcBallCameraPlugin;

impl Plugin for ArcBallCameraPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app
      .init_resource::<MouseDisplacementEventReader>()
      .init_resource::<RotationControlEventReader>()
      .add_startup_system(setup.system())
      .add_system(follow_system.system())
      .add_system(mouse_displacement_system.system())
      .add_system(mouse_rotation_system.system())
      .add_system(rotate_system.system())
      .add_system(zoom_system.system());
  }
}

fn setup(mut commands: Commands) {
  // Target
  let target_entity = commands
    .spawn((
      ArcBallCameraTarget,
      MouseDisplacementControl {
        keyboard_modifier: Some(KeyCode::LShift),
        mouse_modifier: Some(MouseButton::Middle),
        invert_axis: false,
        ..Default::default()
      },
      Rotation::default(),
      Transform::default(),
      Translation::default(),
    ))
    .current_entity();

  // Camera
  let camera_entity = commands
    .spawn(ArcBallCameraComponents {
      control: ArcBallCameraControl {
        target: target_entity,
        distance: 10.0,
        ..Default::default()
      },
      ..Default::default()
    })
    .with(MouseRotationControl {
      keyboard_modifier: Some(KeyCode::LControl),
      mouse_modifier: Some(MouseButton::Middle),
      invert_axis: true,
      movement_threshold: 3.,
    })
    .with(RotationControl {
      target: target_entity,
      ..Default::default()
    })
    .current_entity();

  // Hierarchy
  commands.push_children(target_entity.unwrap(), &[camera_entity.unwrap()]);
}
