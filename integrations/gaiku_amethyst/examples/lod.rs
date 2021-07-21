//! Gaiku Amethyst LOD
//!
//! A small demo of gaiku-amethust
//! This uses the LOD chunker to make a large terrain

const RANGE: f32 = 2000.; // meters in all dimensions
const RESOLUTION: f32 = 0.01; // noise sample per meter

// const RANGE: f32 = 20.; // meters in all dimensions
// const RESOLUTION: f32 = 1.; // noise sample per meter

const CHUNK_DIM: u16 = 16; // The size of the individual chunks in voxels

use amethyst::{
  assets::{AssetStorage, Handle, Loader},
  controls::{FlyControlBundle, FlyControlTag},
  core::{
    math::{Vector3, Vector4},
    transform::{Transform, TransformBundle},
    Hidden,
  },
  ecs::prelude::*,
  input::{InputBundle, StringBindings},
  prelude::*,
  renderer::{
    light::{DirectionalLight, Light},
    palette::{rgb::Rgb, Srgb},
    plugins::{RenderShaded3D, RenderSkybox, RenderToWindow},
    types::{DefaultBackend, MeshData, TextureData},
    visibility::BoundingSphere,
    ActiveCamera, Camera, Material, MaterialDefaults, Mesh, RenderingBundle, Texture,
  },
  ui::{RenderUi, UiBundle},
  utils::application_root_dir,
};

use gaiku_amethyst::prelude::*;
use gaiku_baker_marching_cubes::MarchingCubesBaker as TheBaker;
// use gaiku_baker_voxel::VoxelBaker as TheBaker;
use gaiku_common::{chunk::Chunk, Baker};

use glam::{Mat4, Vec3};
use noise::{Fbm as SourceNoise, NoiseFn, Seedable};
use rand::Rng;

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
    .with_bundle(
      FlyControlBundle::<StringBindings>::new(
        Some(String::from("right")),
        Some(String::from("up")),
        Some(String::from("forward")),
      )
      .with_speed(100.),
    )?
    .with_bundle(input_bundle)?
    .with_bundle(UiBundle::<StringBindings>::new())?;

  let mut game = Application::new(assets_dir, GameLoad::new(), game_data)?;

  game.run();
  Ok(())
}

pub struct GameLoad {
  noise_source: SourceNoise,
  visible_entities: Vec<Entity>,
  terrain_transform: Mat4,
}

impl SimpleState for GameLoad {
  fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
    let world = data.world;

    self.initialise_camera(world);
    self.add_light(world);
    self.build_terrain(world);
  }

  fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
    let world = data.world;
    self.update_visible_chunks(world);
    SimpleTrans::None
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
    Self {
      noise_source: SourceNoise::new().set_seed(0),
      visible_entities: vec![],
      terrain_transform: Mat4::from_scale_rotation_translation(
        // Assuming
        // RANGE = 2000.
        // RESOLUTION = 0.01
        //
        // The size of the noise is 40x40x40
        // The size of the real domain in 4000x4000x4000
        // centered at 2000x2000x2000
        // This transform is to enforce that
        // 0.,0.,0. => 20.,20.,20.
        // 2000.,2000.,2000. => 40.,40.,40.
        // -2000.,-2000.,-2000. => 0.,0.,0.
        [RESOLUTION, RESOLUTION, RESOLUTION].into(),
        Default::default(),
        [RANGE * RESOLUTION, RANGE * RESOLUTION, RANGE * RESOLUTION].into(),
      ),
    }
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

  fn noise(&self, x: f32, y: f32, z: f32) -> f32 {
    const GROUND_HEIGHT: f32 = 0.0; // Could replace with 2D noise
    const HEIGHT_DROPOFF: f32 = 1.0 / 1000.;

    // The fbm noise
    let coords = [x as f64, y as f64, z as f64];
    let noise = self.noise_source.get(coords);

    // Less dense as we go above ground height
    // Just using a linear dropoff but could try exp or power
    let solid_below = -(y - GROUND_HEIGHT) * HEIGHT_DROPOFF;

    noise as f32 + solid_below
  }

  fn build_terrain(&self, world: &mut World) {
    // First we build the noise as an array at lowest LOD
    let density_dimensions = [(RANGE * 2. * RESOLUTION) as usize; 3];

    let origin = [-RANGE, -RANGE, -RANGE];
    let delta = [1. / RESOLUTION, 1. / RESOLUTION, 1. / RESOLUTION];

    let noise_data = (0..density_dimensions[0])
      .into_iter()
      .map(|i| {
        (0..density_dimensions[1])
          .into_iter()
          .map(|j| {
            (0..density_dimensions[2])
              .into_iter()
              .map(|k| {
                self.noise(
                  origin[0] + delta[0] * (i as f32),
                  origin[1] + delta[1] * (j as f32),
                  origin[2] + delta[2] * (k as f32),
                )
              })
              .collect::<Vec<f32>>()
          })
          .collect::<Vec<Vec<f32>>>()
      })
      .collect::<Vec<Vec<Vec<f32>>>>()
      .iter()
      .flatten()
      .flatten()
      .copied()
      .collect::<Vec<f32>>();

    println!("Noise source dimensions: {:?}", density_dimensions);
    let mut chunker: LodChunker<MetaChunk> = LodChunker::from_array(
      &noise_data,
      density_dimensions[0],
      density_dimensions[1],
      density_dimensions[2],
    );

    chunker.set_chunk_size([CHUNK_DIM, CHUNK_DIM, CHUNK_DIM]);
    let (scale, _, _) = self.terrain_transform.to_scale_rotation_translation();
    chunker.set_lod_distance((scale * 50.).length());
    chunker.set_observation_point(
      self
        .terrain_transform
        .transform_point3([0., 0., 0.].into())
        .into(),
    );

    // Some math to make the resolution the same
    // regardless of RANGE/RESOLUTION.
    const MAX_DESIRED_RESOLUTION: f32 = 1.;
    let lods = (((RANGE * 2. / (CHUNK_DIM as f32) / MAX_DESIRED_RESOLUTION).ln() / 2_f32.ln())
      .floor() as usize)
      .max(1);
    println!("Generating: {} lods", lods);
    chunker.set_lod_levels(lods);

    world.insert(chunker);
  }

  fn update_visible_chunks(&mut self, world: &mut World) {
    // First hide all those that are currently shown
    {
      let mut hiddens = world.write_storage::<Hidden>();
      for ent in self.visible_entities.drain(..) {
        if hiddens.insert(ent, Hidden).is_err() {
          println!("Failed to hide old chunk");
        }
      }
    }

    // Now either create or unhide
    type SystemData<'s> = (
      Entities<'s>,
      WriteExpect<'s, LodChunker<MetaChunk>>,
      ReadExpect<'s, Loader>,
      ReadExpect<'s, MaterialDefaults>,
      Read<'s, ActiveCamera>,
      Read<'s, AssetStorage<Mesh>>,
      Read<'s, AssetStorage<Texture>>,
      Read<'s, AssetStorage<Material>>,
      WriteStorage<'s, Transform>,
      WriteStorage<'s, Handle<Mesh>>,
      WriteStorage<'s, Handle<Material>>,
      WriteStorage<'s, BoundingSphere>,
      WriteStorage<'s, Hidden>,
    );
    world.exec(
      |(
        entities,
        mut chunk_tree,
        loader,
        material_default,
        act_cam,
        meshes,
        textures,
        materials,
        mut transforms,
        mut mesh_storage,
        mut material_storage,
        mut bound_storage,
        mut hidden_storage,
      ): SystemData| {
        let inverse_transform = self.terrain_transform.inverse();

        if let Some(cam_ent) = act_cam.entity {
          // Get the cameras global position
          let global_cam_pos = {
            let cam_trans = transforms
              .get(cam_ent)
              .expect("Camera should have a transform");
            cam_trans.global_matrix() * Vector4::new(0., 0., 0., 1.0)
          };

          // Update observation_point
          chunk_tree.set_observation_point(
            self
              .terrain_transform
              .transform_point3([global_cam_pos[0], global_cam_pos[1], global_cam_pos[2]].into())
              .into(),
          );

          // Generate the chunks from this observation_point
          chunk_tree.generate_chunks();

          // Make chunks if visible
          for chunked in chunk_tree.get_chunks_mut() {
            // let scale: [f32; 3] = chunked.scale;
            let chunk = &mut chunked.chunk;

            if let Some(ent) = chunk.get_entity() {
              // Chunk already has an entity just use that
              hidden_storage.remove(ent);
              self.visible_entities.push(ent);
            } else {
              // This chunk is made up of this many voxel
              let chunk_size: Vec3 = [
                (chunk.width() - 1) as f32,
                (chunk.height() - 1) as f32,
                (chunk.depth() - 1) as f32,
              ]
              .into();
              // Total area the chunk tree covers is this:
              let area_size: Vec3 = [RANGE * 2., RANGE * 2., RANGE * 2.].into();

              let origin: [f32; 3] = inverse_transform
                .transform_point3(chunked.location.into())
                .into();
              let scale: [f32; 3] = (Vec3::from(chunked.scale) * area_size / chunk_size).into();

              // Create new entitiy for this chunk
              // let color = [10, 200, 10, 255];
              let color = [
                rand::thread_rng().gen_range(0..255),
                rand::thread_rng().gen_range(0..255),
                rand::thread_rng().gen_range(0..255),
                255,
              ];

              let bake_result = self.make_mesh_from_chunk(chunk, color);

              // Make an entity we can assign to the metachunk
              let entity = if let Some((mesh_data, tex_data)) = bake_result {
                // if the bake was successful this entity will use the baked mesh
                let (mesh, mat) = {
                  let tex = loader.load_from_data(tex_data, (), &textures);
                  let mesh = loader.load_from_data(mesh_data, (), &meshes);
                  let mat: Handle<Material> = loader.load_from_data(
                    Material {
                      albedo: tex,
                      ..material_default.0.clone()
                    },
                    (),
                    &materials,
                  );
                  (mesh, mat)
                };

                // Get transform from the tree data
                let mut transform = Transform::default();
                transform.set_translation_xyz(origin[0], origin[1], origin[2]);

                transform.set_scale(scale.into());
                let current_size: Vec3 = [
                  chunk.width() as f32,
                  chunk.height() as f32,
                  chunk.depth() as f32,
                ]
                .into();

                // The bounding box should also be set so that amethyst dosen't clip it
                // These are in mesh coordinates before scaling
                let radius = current_size.length() * 1.5;
                let center: [f32; 3] = (current_size / 2.).into();
                let bounding = BoundingSphere {
                  center: center.into(),
                  radius,
                };

                entities
                  .build_entity()
                  .with(mesh, &mut mesh_storage)
                  .with(mat, &mut material_storage)
                  .with(transform, &mut transforms)
                  .with(bounding, &mut bound_storage)
                  .build()
              } else {
                // Otherwise we just assign an empty entity
                entities.build_entity().build()
              };
              chunk.set_entity(entity);
              self.visible_entities.push(entity);
            }
          }
        }
      },
    );
  }

  fn make_mesh_from_chunk(
    &self,
    chunk: &MetaChunk,
    color: [u8; 4],
  ) -> Option<(MeshData, TextureData)> {
    // Make a texture that just has a green tile in it
    let mut texture = TextureAtlas2d::new(4);
    texture.set_at_index(0, [color; 4 * 4].to_vec());

    // Create the baker options to include this texture
    let options = BakerOptions {
      texture: Some(texture),
      ..Default::default()
    };

    // Bake the mesh
    let meshgox = TheBaker::bake::<MetaChunk, GaikuTexture2d, GaikuMesh>(chunk, &options);

    if let Ok(Some(mesh)) = meshgox {
      let tex = options.texture.unwrap().get_texture();

      // Put all data into amethyst format
      let tex_data: TextureData = tex.into();
      let mesh_data: MeshData = mesh.into();
      Some((mesh_data, tex_data))
    } else {
      // Nothing too bake probably an empty chunk
      None
    }
  }
}

/// We use this structure for the LOD tree
/// chunks so that we can hold extra meta data
/// with it related to amethyst and the mesh
pub struct MetaChunk {
  chunk: Chunk,
  entity: Option<Entity>,
}

impl MetaChunk {
  pub fn get_entity(&self) -> Option<Entity> {
    self.entity
  }

  pub fn set_entity(&mut self, ent: Entity) {
    self.entity = Some(ent);
  }
}

impl Boxify for MetaChunk {
  fn new(position: [f32; 3], width: u16, height: u16, depth: u16) -> Self {
    Self {
      chunk: Chunk::new(position, width, height, depth),
      entity: None,
    }
  }
}

impl Chunkify<f32> for MetaChunk {
  fn is_air(&self, x: usize, y: usize, z: usize, isovalue: f32) -> bool {
    self.chunk.is_air(x, y, z, isovalue)
  }

  fn get(&self, x: usize, y: usize, z: usize) -> f32 {
    self.chunk.get(x, y, z)
  }
}

impl ChunkifyMut<f32> for MetaChunk {
  fn set(&mut self, x: usize, y: usize, z: usize, value: f32) {
    self.chunk.set(x, y, z, value)
  }
}

impl Atlasify<u8> for MetaChunk {
  fn get_atlas(&self, x: usize, y: usize, z: usize) -> u8 {
    self.chunk.get_atlas(x, y, z)
  }
}

impl AtlasifyMut<u8> for MetaChunk {
  fn set_atlas(&mut self, x: usize, y: usize, z: usize, value: u8) {
    self.chunk.set_atlas(x, y, z, value)
  }
}

impl Positionable for MetaChunk {
  fn with_position(position: [f32; 3]) -> Self {
    Self {
      chunk: Chunk::new(position, 16, 16, 16),
      entity: None,
    }
  }

  fn position(&self) -> [f32; 3] {
    self.chunk.position()
  }
}

impl Sizable for MetaChunk {
  fn with_size(width: u16, height: u16, depth: u16) -> Self {
    Self {
      chunk: Chunk::new([0.0, 0.0, 0.0], width, height, depth),
      entity: None,
    }
  }

  fn depth(&self) -> u16 {
    self.chunk.depth()
  }

  fn height(&self) -> u16 {
    self.chunk.height()
  }

  fn width(&self) -> u16 {
    self.chunk.width()
  }
}
