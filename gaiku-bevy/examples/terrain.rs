use bevy::prelude::*;
use gaiku_bevy::*;

pub fn setup(
  commands: &mut Commands,
  asset_server: Res<AssetServer>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  commands
    .spawn(PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
      material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
      ..Default::default()
    })
    .spawn(Camera3dBundle {
      transform: Transform::from_translation(Vec3::new(0.0, 120.0, -200.0))
        .looking_at(Vec3::default(), Vec3::unit_y()),
      ..Default::default()
    });

  commands
    .spawn_scene(asset_server.load("terrain.gox"))
    .spawn(LightBundle {
      transform: Transform::from_translation(Vec3::new(4., 8., 4.)),
      ..Default::default()
    });
}

fn main() {
  App::build()
    .add_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_plugin(GaikuPlugin)
    .add_startup_system(setup.system())
    .run();
}
