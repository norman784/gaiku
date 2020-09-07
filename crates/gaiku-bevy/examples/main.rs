use bevy::{
  app::AppExit,
  prelude::*,
  render::{mesh::VertexAttribute, pipeline::PrimitiveTopology},
};
use gaiku::{
  bakers::{HeightMapBaker, MarchingCubesBaker, VoxelBaker},
  common::{self, Baker, FileFormat},
  formats::{GoxReader, PNGReader},
};

mod plugins;
use plugins::fly_camera::*;

struct MainCamera;

#[derive(Debug)]
enum CameraType {
  DefaultCamera,
  FlyCamera,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum BakerType {
  Heightmap,
  MarchingCubes,
  None,
  Voxel,
}

#[derive(Debug)]
struct Settings {
  baker: BakerType,
  camera: CameraType,
}

impl Default for Settings {
  fn default() -> Self {
    Self {
      baker: BakerType::None,
      camera: CameraType::DefaultCamera,
    }
  }
}

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
// TODO: Implement custom camera plugin
fn change_camera_system(mut settings: ResMut<Settings>, input: Res<Input<KeyCode>>) {
  if input.just_released(KeyCode::Tab) {
    match settings.camera {
      CameraType::FlyCamera => settings.camera = CameraType::DefaultCamera,
      _ => settings.camera = CameraType::FlyCamera,
    }
    println!("Current camera {:?}", settings.camera);
  }
}

fn change_baker_system(
  mut commands: Commands,
  mut settings: ResMut<Settings>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  input: Res<Input<KeyCode>>,
) {
  let mut loaded_meshes = vec![];

  if input.pressed(KeyCode::Key1) && settings.baker != BakerType::Voxel {
    settings.baker = BakerType::Voxel;

    let file = format!(
      "{}/../../examples/assets/{}.gox",
      env!("CARGO_MANIFEST_DIR"),
      "small_tree"
    );

    println!("Reading file: {}", &file);
    let chunks = GoxReader::read(&file);

    for chunk in chunks.iter() {
      //let mesh = MarchingCubesBaker::bake(chunk);
      let mesh = VoxelBaker::bake(chunk);
      if let Some(mesh) = mesh {
        loaded_meshes.push((IMesh::from(mesh), chunk.position()));
      }
    }
    println!("Baked {} meshes", loaded_meshes.len());
  } else if input.pressed(KeyCode::Key2) && settings.baker != BakerType::MarchingCubes {
    settings.baker = BakerType::MarchingCubes;

    let file = format!(
      "{}/../../examples/assets/{}.gox",
      env!("CARGO_MANIFEST_DIR"),
      "small_tree"
    );

    println!("Reading file: {}", &file);
    let chunks = GoxReader::read(&file);

    for chunk in chunks.iter() {
      let mesh = MarchingCubesBaker::bake(chunk);
      if let Some(mesh) = mesh {
        loaded_meshes.push((IMesh::from(mesh), chunk.position()));
      }
    }
    println!("Baked {} meshes", loaded_meshes.len());
  } else if input.pressed(KeyCode::Key3) && settings.baker != BakerType::Heightmap {
    settings.baker = BakerType::Heightmap;

    let file = format!(
      "{}/../../examples/assets/{}.png",
      env!("CARGO_MANIFEST_DIR"),
      "heightmap"
    );

    println!("Reading file: {}", &file);
    let chunks = PNGReader::read(&file);

    for chunk in chunks.iter() {
      let mesh = HeightMapBaker::bake(chunk);
      if let Some(mesh) = mesh {
        loaded_meshes.push((IMesh::from(mesh), chunk.position()));
      }
    }
    println!("Baked {} meshes", loaded_meshes.len());
  }

  if loaded_meshes.len() > 0 {
    println!("Current Baker: {:?}", settings.baker);

    for (mesh, position) in loaded_meshes.iter() {
      commands.spawn(PbrComponents {
        mesh: meshes.add(Mesh::from(mesh.clone())),
        material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
        translation: Translation::new(position[0], position[1], position[2]),
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

fn setup(mut commands: Commands) {
  // add entities to the world
  commands
    /*
    .spawn(Camera3dComponents {
      translation: Translation::new(10.0, 5.0, 30.0),
      rotation: Rotation::from_rotation_xyz(-0.3, 0.5, 0.0),
      ..Default::default()
    })
    */
    .spawn(FlyCamera::default())
    .spawn(LightComponents {
      translation: Translation::new(4.0, 8.0, 4.0),
      ..Default::default()
    })
    .with(MainCamera);
}

// MAIN
fn main() {
  App::build()
    .add_resource(Msaa { samples: 4 })
    .add_resource(Settings::default())
    .add_default_plugins()
    .add_plugin(FlyCameraPlugin)
    .add_startup_system(setup.system())
    .add_system(exit_app_system.system())
    .add_system(change_camera_system.system())
    .add_system(change_baker_system.system())
    .run();
}
