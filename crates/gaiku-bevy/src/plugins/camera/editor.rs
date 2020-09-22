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
pub struct EditorCameraComponents {
  pub control: EditorCameraControl,
  pub camera: Camera,
  pub perspective_projection: PerspectiveProjection,
  pub visible_entities: VisibleEntities,
  pub transform: Transform,
  pub global_transform: GlobalTransform,
}

impl Default for EditorCameraComponents {
  fn default() -> Self {
    Self {
      control: EditorCameraControl {
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
      global_transform: Default::default(),
    }
  }
}

// ==============
// COMPONENTS
// ==============

pub struct EditorCameraTarget;

#[derive(Default)]
pub struct EditorCameraControl {
  pub distance: f32,
  pub pitch: f32,
  pub yaw: f32,
  pub target: Option<Entity>,
}

// ==============
// SYSTEMS
// ==============

fn follow_system(
  mut query: Query<(&EditorCameraControl, &mut Transform)>,
  target_query: Query<&Transform>,
) {
  for (control, mut transform) in &mut query.iter() {
    if let Some(target) = control.target {
      if let Ok(target_transform) = target_query.get::<Transform>(target) {
        let position = target_transform.translation()
          - (transform.rotation() * -Vec3::unit_z() * control.distance);
        transform.set_translation(position);
      }
    }
  }
}

// ==============
// PLUGINS
// ==============

pub struct EditorCameraPlugin;

impl Plugin for EditorCameraPlugin {
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
      EditorCameraTarget,
      MouseDisplacementControl {
        keyboard_modifier: Some(KeyCode::LShift),
        mouse_modifier: Some(MouseButton::Middle),
        invert_axis: false,
        ..Default::default()
      },
      Transform::default(),
      GlobalTransform::default(),
    ))
    .current_entity();

  // Camera
  let camera_entity = commands
    .spawn(EditorCameraComponents {
      control: EditorCameraControl {
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
