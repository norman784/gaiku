//! Gaiku Amethyst Terrain
//!
//! A small demo of gaiku-amethust
//! using a terrain goxel.

use amethyst::{
  assets::{Handle, Loader},
  controls::{FlyControlBundle, FlyControlTag},
  core::{
    math::{Matrix4, Vector3, Vector4},
    transform::{Transform, TransformBundle},
  },
  ecs::prelude::*,
  input::{InputBundle, StringBindings},
  prelude::*,
  renderer::{
    light::{DirectionalLight, Light},
    palette::{rgb::Rgb, Srgb},
    plugins::{RenderShaded3D, RenderSkybox, RenderToWindow},
    types::{DefaultBackend, TextureData},
    visibility::BoundingSphere,
    ActiveCamera, Camera, Material, MaterialDefaults, Mesh, RenderingBundle,
  },
  ui::{RenderUi, UiBundle},
  utils::application_root_dir,
};

use gaiku_baker_marching_cubes::MarchingCubesBaker;
use gaiku_baker_modified_marching_cubes::ModMarchingCubesBaker;
use gaiku_baker_voxel::VoxelBaker;
use gaiku_common::{chunk::Chunk, Baker};
use gaiku_format_gox::GoxReader;

use gaiku_amethyst::prelude::*;

enum BakerSelect {
  Voxel,
  Marching,
  ModMarching,
}

fn main() -> amethyst::Result<()> {
  amethyst::start_logger(Default::default());

  let app_root = application_root_dir()?;
  let assets_dir = app_root.join("examples").join("assets");

  let display_config_path = assets_dir.join("display.ron");

  let binding_path = assets_dir.join("bindings.ron");
  let input_bundle = InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

  let render_bund = RenderingBundle::<DefaultBackend>::new()
    // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
    .with_plugin(
      RenderToWindow::from_config_path(display_config_path)?.with_clear([0.0, 0.0, 0.0, 1.0]),
    )
    .with_plugin(RenderShaded3D::default())
    .with_plugin(RenderUi::default())
    .with_plugin(RenderSkybox::with_colors(
      Srgb::new(0.82, 0.51, 0.50),
      Srgb::new(0.18, 0.11, 0.85),
    ));

  let game_data = GameDataBuilder::default()
    .with_bundle(render_bund)?
    // With transform systems for position tracking
    .with_bundle(TransformBundle::new())?
    .with_bundle(FlyControlBundle::<StringBindings>::new(
      Some(String::from("right")),
      Some(String::from("up")),
      Some(String::from("forward")),
    ))?
    .with_bundle(input_bundle)?
    .with_bundle(UiBundle::<StringBindings>::new())?;

  let mut game = Application::new(assets_dir, GameLoad::new(), game_data)?;

  game.run();
  Ok(())
}

pub struct GameLoad;

impl SimpleState for GameLoad {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;

    self.initialise_camera(world);
    self.add_light(world);
    self.build_terrain(world, BakerSelect::Voxel, [-12., 0., 0.]);
    self.build_terrain(world, BakerSelect::Marching, [0., 0., 0.]);
    self.build_terrain(world, BakerSelect::ModMarching, [12., 0., 0.]);
  }

  // Uncomment this to print the camera position.
  // Useful to find where you might want to save the camera position
  //
  // fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
  //   let world = data.world;
  //   type SystemData<'s> = (Read<'s, ActiveCamera>, ReadStorage<'s, Transform>);
  //   world.exec(|(act_cam, transforms): SystemData| {
  //     if let Some(act_cam_ent) = act_cam.entity {
  //       if let Some(cam_trans) = transforms.get(act_cam_ent) {
  //         println!("Cam Location: {:?}", cam_trans.translation());
  //       }
  //     }
  //   });
  //   SimpleTrans::None
  // }
}

impl Default for GameLoad {
  fn default() -> Self {
    Self::new()
  }
}

impl GameLoad {
  pub fn new() -> Self {
    Self {}
  }

  fn initialise_camera(&self, world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 10., 22.0);
    transform.face_towards(Vector3::new(0., 5., 0.), Vector3::new(0., 1., 0.));

    let cam_ent = world
      .create_entity()
      .with(Camera::standard_3d(600., 400.))
      .with(transform)
      .with(FlyControlTag)
      .build();
    let act_cam: &mut ActiveCamera = world.get_mut().expect("There shoud be an active camera");
    act_cam.entity = Some(cam_ent);
  }

  fn add_light(&self, world: &mut World) {
    world
      .create_entity()
      .with(Light::from(DirectionalLight {
        color: Rgb::new(1.0, 1.0, 1.0),
        direction: [-1.0, -1.0, -1.0].into(),
        intensity: 1.0,
      }))
      .build();
  }

  fn build_terrain(&self, world: &mut World, baker: BakerSelect, location: [f32; 3]) {
    let file = format!(
      "{}/examples/assets/{}.gox",
      env!("CARGO_MANIFEST_DIR"),
      "terrain"
    );
    let text_file = match baker {
      BakerSelect::Voxel => format!(
        "{}/examples/assets/{}.gox",
        env!("CARGO_MANIFEST_DIR"),
        "voxel_text"
      ),
      BakerSelect::Marching => format!(
        "{}/examples/assets/{}.gox",
        env!("CARGO_MANIFEST_DIR"),
        "marching_text"
      ),
      BakerSelect::ModMarching => format!(
        "{}/examples/assets/{}.gox",
        env!("CARGO_MANIFEST_DIR"),
        "mod_marching_text"
      ),
    };

    let (chunks, texture) = GoxReader::read::<Chunk, GaikuTexture2d>(&file).unwrap();
    let options = BakerOptions {
      texture,
      ..Default::default()
    };
    let mut meshes = vec![];

    for chunk in chunks.iter() {
      let mesh = match baker {
        BakerSelect::Voxel => {
          VoxelBaker::bake::<Chunk, GaikuTexture2d, GaikuMesh>(chunk, &options).unwrap()
        }
        BakerSelect::Marching => {
          MarchingCubesBaker::bake::<Chunk, GaikuTexture2d, GaikuMesh>(chunk, &options).unwrap()
        }
        BakerSelect::ModMarching => {
          ModMarchingCubesBaker::bake::<Chunk, GaikuTexture2d, GaikuMesh>(chunk, &options).unwrap()
        }
      };
      let dimension = [
        chunk.width() as f32,
        chunk.height() as f32,
        chunk.depth() as f32,
      ];
      if let Some(mesh) = mesh {
        meshes.push((mesh, chunk.position(), dimension, &options));
      }
    }

    let (text_chunks, text_texture) = GoxReader::read::<Chunk, GaikuTexture2d>(&text_file).unwrap();
    let text_options = BakerOptions {
      texture: text_texture,
      ..Default::default()
    };

    for chunk in text_chunks.iter() {
      let mesh = match baker {
        BakerSelect::Voxel => {
          VoxelBaker::bake::<Chunk, GaikuTexture2d, GaikuMesh>(chunk, &text_options).unwrap()
        }
        BakerSelect::Marching => {
          MarchingCubesBaker::bake::<Chunk, GaikuTexture2d, GaikuMesh>(chunk, &text_options)
            .unwrap()
        }
        BakerSelect::ModMarching => {
          ModMarchingCubesBaker::bake::<Chunk, GaikuTexture2d, GaikuMesh>(chunk, &text_options)
            .unwrap()
        }
      };
      let dimension = [
        chunk.width() as f32,
        chunk.height() as f32,
        chunk.depth() as f32,
      ];
      let mut text_pos = chunk.position();
      text_pos[1] += 100.;
      if let Some(mesh) = mesh {
        meshes.push((mesh, text_pos, dimension, &text_options));
      }
    }

    let scale = Vector3::new(0.1, 0.1, 0.1);
    let swap_axes = true;
    let transform = if swap_axes {
      Matrix4::from_euler_angles(0., 0., 0.)
    } else {
      Matrix4::identity()
    };
    for (mut mesh_gox, position, dimension, options) in meshes {
      let tex_data: TextureData = options.texture.as_ref().unwrap().get_texture().into();
      let (mesh, mat) = {
        if swap_axes {
          // Swap y/z for amethyst coordinate system
          mesh_gox.positions = mesh_gox
            .positions
            .iter()
            .map(|vert| {
              let v = Vector4::new(vert[0], vert[1], vert[2], 1.);
              let vtran = transform * v;
              [vtran[0], vtran[1], vtran[2]]
            })
            .collect::<Vec<_>>();

          mesh_gox.normals = mesh_gox
            .normals
            .iter()
            .map(|normal| {
              let v = Vector4::new(normal[0], normal[1], normal[2], 1.);
              let vtran = transform * v;
              [vtran[0], vtran[1], vtran[2]]
            })
            .collect::<Vec<_>>();
        }
        let loader = world.read_resource::<Loader>();
        let mat_default = world.read_resource::<MaterialDefaults>();
        let mesh_data = mesh_gox.into();
        let mesh: Handle<Mesh> = loader.load_from_data(mesh_data, (), &world.read_resource());
        let tex = loader.load_from_data(tex_data.clone(), (), &world.read_resource());
        let mat: Handle<Material> = loader.load_from_data(
          Material {
            albedo: tex,
            ..mat_default.0.clone()
          },
          (),
          &world.read_resource(),
        );
        (mesh, mat)
      };
      let mut pos = Transform::default();

      let position_trans = {
        let v = Vector4::new(position[0], position[1], position[2], 1.);
        let vtrans = transform * v;
        Vector3::new(
          vtrans[0] * scale[0] + location[0],
          vtrans[1] * scale[1] + location[1],
          vtrans[2] * scale[2] + location[2],
        )
      };
      pos.set_translation(position_trans);
      pos.set_scale(scale);

      let radius =
        (dimension[0].powi(2) + dimension[1].powi(2) + dimension[2].powi(2)).sqrt() * 1.5;
      let bounding = BoundingSphere {
        center: [dimension[0] / 2., dimension[1] / 2., dimension[2] / 2.].into(),
        radius,
      };

      let _voxel = world
        .create_entity()
        .with(mesh)
        .with(mat)
        .with(pos)
        .with(bounding)
        .build();
    }
  }
}
