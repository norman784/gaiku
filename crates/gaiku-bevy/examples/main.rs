use bevy::{
  app::AppExit,
  prelude::*,
  render::{mesh::VertexAttribute, pipeline::PrimitiveTopology},
};
use gaiku::{
  bakers::Voxel,
  common::{self, Baker, FileFormat},
  formats::Gox,
};
use gaiku_bevy::plugins::*;

#[derive(Clone)]
pub struct IMesh {
  pub indices: Vec<u32>,
  pub vertices: Vec<common::Vector3>,
  pub normals: Vec<common::Vector3>,
  pub colors: Vec<common::Color>,
  pub uv: Vec<common::Vector2>,
}

impl IMesh {
  fn from(mesh: common::Mesh) -> Self {
    Self {
      indices: mesh.indices,
      vertices: mesh.vertices,
      normals: mesh.normals,
      colors: mesh.colors,
      uv: mesh.uv,
    }
  }
}

impl From<IMesh> for Mesh {
  fn from(mesh: IMesh) -> Self {
    Mesh {
      primitive_topology: PrimitiveTopology::TriangleList,
      attributes: vec![
        VertexAttribute::position(mesh.vertices),
        VertexAttribute::normal(mesh.normals),
        VertexAttribute::uv(mesh.uv),
      ],
      indices: Some(mesh.indices),
    }
  }
}

//SYSTEMS
fn change_baker_system(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  input: Res<Input<KeyCode>>,
) {
  let mut loaded_meshes = vec![];

  if input.pressed(KeyCode::Key1) {
    let file = format!(
      "{}/../../examples/assets/{}.gox",
      env!("CARGO_MANIFEST_DIR"),
      "small_tree"
    );

    println!("Reading file: {}", &file);
    //let chunks = Gox::read(&file);
    let chunks = vec![];

    for chunk in chunks.iter() {
      //let mesh = MarchingCubesBaker::bake(chunk);
      let mesh = Voxel::bake(chunk);
      if let Some(mesh) = mesh {
        loaded_meshes.push((IMesh::from(mesh), chunk.position()));
      }
    }
    println!("Baked {} meshes", loaded_meshes.len());
  }

  if loaded_meshes.len() > 0 {
    for (mesh, position) in loaded_meshes.iter() {
      let [x, y, z] = *position;
      commands.spawn(PbrComponents {
        mesh: meshes.add(Mesh::from(mesh.clone())),
        material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
        transform: Transform::from_translation(Vec3::new(x, y, z)),
        ..Default::default()
      });
    }
  }
}

fn exit_app_system(input: Res<Input<KeyCode>>, mut event: ResMut<Events<AppExit>>) {
  if input.pressed(KeyCode::Escape) {
    event.send(AppExit);
  }
}

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  // AssetLoader
  /*
  commands.spawn(PbrComponents {
    mesh: asset_server.load("assets/small_tree.gox").unwrap(),
    material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
    transform: Transform::from_translation(Vec3::new(-1.5, 0.0, 0.0)),
    ..Default::default()
  });
    */
  // Spawn temporary stuff
  commands
    // Plane
    .spawn(PbrComponents {
      mesh: meshes.add(Mesh::from(shape::Plane { size: 10. })),
      material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
      ..Default::default()
    })
    // cube
    .spawn(PbrComponents {
      mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
      material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
      transform: Transform::from_translation(Vec3::new(0., 1., 0.)),
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
    //.add_system(change_baker_system.system())
    .run();
}
