use bevy::{
  prelude::*,
  render::{mesh::VertexAttribute, pipeline::PrimitiveTopology},
};
use gaiku::{
  bakers::Voxel,
  common::{Baker, FileFormat},
  formats::Gox,
};
use gaiku_bevy::plugins::*;

use common::exit_app_system;
mod common;

struct ChunkTag;

//SYSTEMS
fn spawn_model_system(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  input: Res<Input<KeyCode>>,
) {
  let file_name = if input.pressed(KeyCode::Key1) {
    Some("small_tree")
  } else if input.pressed(KeyCode::Key2) {
    Some("terrain")
  } else if input.pressed(KeyCode::Key3) {
    Some("planet")
  } else {
    None
  };

  if let Some(file_name) = file_name {
    let path = format!(
      "{}/../../examples/assets/{}.gox",
      env!("CARGO_MANIFEST_DIR"),
      file_name
    );

    if let Ok(chunks) = Gox::load_file(&path) {
      println!("file {:?} loaded {:?} chunks", file_name, chunks.len());
      chunks.iter().for_each(|c| {
        if let Some(m) = Voxel::bake(c) {
          let [x, y, z] = c.position();
          let mesh = Mesh {
            primitive_topology: PrimitiveTopology::TriangleList,
            attributes: vec![
              VertexAttribute::position(m.vertices.clone()),
              VertexAttribute::normal(m.normals.clone()),
              VertexAttribute::uv(m.uv.clone()),
            ],
            indices: Some(m.indices.clone()),
          };
          commands
            .spawn(PbrComponents {
              mesh: meshes.add(mesh),
              material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
              transform: Transform::from_translation(Vec3::new(x, y, z)),
              ..Default::default()
            })
            .with(ChunkTag);
        }
      });
    } else {
      println!("failed to load: {:?}", path);
    }
  }
}

fn clear_models_system(
  mut commands: Commands,
  input: Res<Input<KeyCode>>,
  mut query: Query<(Entity, &ChunkTag)>,
) {
  if input.pressed(KeyCode::Space) {
    for (entity, _) in &mut query.iter() {
      commands.despawn(entity);
    }
  }
}

fn setup(mut commands: Commands) {
  commands.spawn(LightComponents {
    transform: Transform::from_translation(Vec3::new(4., 8., 100.)),
    ..Default::default()
  });
}

// MAIN
fn main() {
  App::build()
    .add_resource(Msaa { samples: 4 })
    .add_default_plugins()
    .add_plugin(camera::editor::EditorCameraPlugin)
    .add_startup_system(setup.system())
    .add_system(exit_app_system.system())
    .add_system(spawn_model_system.system())
    .add_system(clear_models_system.system())
    .run();
}
