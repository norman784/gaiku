use bevy::prelude::*;
use gaiku_bevy::plugins::*;

use common::exit_app_system;
mod common;

//SYSTEMS

fn setup(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // AssetLoader
  commands.spawn(PbrComponents {
    mesh: asset_server.load("assets/planet.gox").unwrap(),
    material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
    transform: Transform::from_translation(Vec3::new(40., 0., 20.)),
    ..Default::default()
  });

  commands.spawn(PbrComponents {
    mesh: asset_server.load("assets/small_tree.gox").unwrap(),
    material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
    transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
    ..Default::default()
  });

  commands.spawn(PbrComponents {
    mesh: asset_server.load("assets/terrain.gox").unwrap(),
    material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
    transform: Transform::from_translation(Vec3::new(-80., 30., 20.)),
    ..Default::default()
  });

  // Default setup
  commands.spawn(LightComponents {
    transform: Transform::from_translation(Vec3::new(4., 8., 4.)),
    ..Default::default()
  });
}

// MAIN
fn main() {
  App::build()
    .add_resource(Msaa { samples: 4 })
    .add_default_plugins()
    .add_plugin(loaders::GoxPlugin)
    .add_plugin(camera::editor::EditorCameraPlugin)
    .add_startup_system(setup.system())
    .add_system(exit_app_system.system())
    .run();
}
