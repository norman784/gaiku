use anyhow;
use bevy_app::{AppBuilder, Plugin};
use bevy_asset::{AddAsset, AssetLoader, AssetPath, LoadContext, LoadedAsset};
use bevy_ecs::prelude::{World, WorldBuilderSource};
use bevy_pbr::prelude::{PbrBundle, StandardMaterial};
use bevy_render::prelude::{Color, Mesh, Texture};
use bevy_scene::Scene;
use bevy_transform::prelude::{BuildWorldChildren, GlobalTransform, Transform};
use bevy_utils::BoxedFuture;
use gaiku_3d::{bakers::VoxelBaker, formats::GoxReader};
use gaiku_common::{chunk::Chunk, prelude::*};

use crate::{GaikuMesh, GaikuTexture};

#[derive(Default)]
pub struct GaikuPlugin;

impl Plugin for GaikuPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.init_asset_loader::<GaikuAssetLoader>();
  }
}

#[derive(Default)]
pub struct GaikuAssetLoader;

impl AssetLoader for GaikuAssetLoader {
  fn load<'a>(
    &'a self,
    bytes: &'a [u8],
    load_context: &'a mut LoadContext,
  ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
    Box::pin(async move {
      let mut world = World::default();
      let world_builder = &mut world.build();

      let (chunks, atlas): (Vec<Chunk>, Option<TextureAtlas2d<GaikuTexture>>) =
        GoxReader::load(bytes.to_vec())?;

      let texture_label = "test_texture.png";

      let loaded_asset = if let Some(atlas) = &atlas {
        let texture: Texture = atlas.get_texture().into();
        LoadedAsset::new(texture)
      } else {
        LoadedAsset::new(Texture::default())
      };

      load_context.set_labeled_asset(texture_label, loaded_asset);

      let material_label = "ChunkAtlas";
      load_context.set_labeled_asset(
        material_label,
        LoadedAsset::new(StandardMaterial {
          albedo: Color::WHITE,
          albedo_texture: Some(load_context.get_handle(AssetPath::new_ref(
            load_context.path(),
            Some(material_label),
          ))),
          ..Default::default()
        }),
      );

      let baker_options = BakerOptions {
        texture: atlas,
        ..Default::default()
      };

      for chunk in chunks.iter() {
        let mesh: Option<GaikuMesh> = VoxelBaker::bake(chunk, &baker_options)?;
        if let Some(mesh) = mesh {
          let mesh: Mesh = mesh.into();

          let name = format!("Chunk{:?}", chunk.position());
          load_context.set_labeled_asset(&name, LoadedAsset::new(mesh));
        }
      }

      world_builder
        .spawn((
          //Transform::from_rotation(Quat::from_rotation_x(-90.0)),
          Transform::default(),
          GlobalTransform::default(),
        ))
        .with_children(|parent| {
          for chunk in chunks.iter() {
            let name = format!("Chunk[{:?}]", &chunk.position());
            let mesh_asset_path = AssetPath::new_ref(load_context.path(), Some(&name));
            let material_asset_path = AssetPath::new_ref(load_context.path(), Some(material_label));

            parent.spawn(PbrBundle {
              mesh: load_context.get_handle(mesh_asset_path),
              material: load_context.get_handle(material_asset_path),
              transform: Transform::from_translation(chunk.position().into()),
              ..Default::default()
            });
          }
        });

      load_context.set_default_asset(LoadedAsset::new(Scene::new(world)));

      Ok(())
    })
  }

  fn extensions(&self) -> &[&str] {
    &["gox"]
  }
}
