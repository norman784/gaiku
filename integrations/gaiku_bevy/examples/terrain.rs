use bevy::prelude::*;
use gaiku_common::chunk::Chunk;
use gaiku_baker_voxel::VoxelBaker;
use gaiku_bevy::*;

struct CameraRotation {
  angle: f32,
}

fn setup(commands: &mut Commands, asset_server: Res<AssetServer>) {
  commands
    .spawn((
      Transform::default(),
      GlobalTransform::default(),
      CameraRotation { angle: 0.0 },
    ))
    .with_children(|parent| {
      parent.spawn(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 120.0, -200.0))
          .looking_at(Vec3::default(), Vec3::unit_y()),
        ..Default::default()
      });
    })
    .spawn_scene(asset_server.load("terrain.gox"))
    .spawn(LightBundle {
      transform: Transform::from_translation(Vec3::new(0.0, 50.0, 50.0)),
      ..Default::default()
    });
}

fn rotate_terrain(time: Res<Time>, mut query: Query<(&mut CameraRotation, &mut Transform)>) {
  for (mut rotation, mut transform) in &mut query.iter_mut() {
    rotation.angle += 10.0 * time.delta_seconds();
    transform.rotation = Quat::from_rotation_y(rotation.angle.to_radians());
  }
}

fn main() {
  App::build()
    .add_resource(Msaa { samples: 4 })
    .add_plugins(DefaultPlugins)
    .add_plugin(GaikuPlugin::<GoxReader, VoxelBaker, Chunk, (u8, u8)>::default())
    .add_startup_system(setup.system())
    .add_system(rotate_terrain.system())
    .run();
}
