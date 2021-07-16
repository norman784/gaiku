use std::marker::PhantomData;

use anyhow;
use bevy_app::{AppBuilder, Plugin};
use bevy_asset::{AddAsset, AssetLoader, AssetPath, LoadContext, LoadedAsset};
use bevy_ecs::prelude::World;
use bevy_pbr::prelude::{PbrBundle, StandardMaterial};
use bevy_render::prelude::{Color, Mesh, Texture};
use bevy_scene::Scene;
use bevy_transform::prelude::Transform;
use bevy_utils::BoxedFuture;
use gaiku_common::prelude::*;

use crate::{GaikuMesh, GaikuTexture}; 

#[derive(Default)]
pub struct GaikuPlugin<F, B, C>
where
  F: FileFormat + Send + Sync + 'static + Default,
  B: Baker + Send + Sync + 'static + Default,
  C: Chunkify<F::Value> + ChunkifyMut<F::Value> + Atlasify<B::AtlasValue> + AtlasifyMut<B::AtlasValue> + Boxify + Sizable + Send + Sync + 'static + Default,
{
  file_format: PhantomData<F>,
  baker: PhantomData<B>,
  chunk: PhantomData<C>,
}

impl<F, B, C> Plugin for GaikuPlugin<F, B, C>
where
  F: FileFormat + Send + Sync + 'static + Default,
  B: Baker + Send + Sync + 'static + Default,
  C: Chunkify<F::Value> + ChunkifyMut<F::Value> + Atlasify<B::AtlasValue> + AtlasifyMut<B::AtlasValue> + Boxify + Sizable + Send + Sync + 'static + Default,
{
  fn build(&self, app: &mut AppBuilder) {
    app.init_asset_loader::<GaikuAssetLoader<F, B, C>>();
  }
}

#[derive(Default)]
pub struct GaikuAssetLoader<F, B, C>
where
  F: FileFormat + Send + Sync + 'static + Default,
  B: Baker + Send + Sync + 'static + Default,
  C: Chunkify<F::Value> + ChunkifyMut<F::Value> + Atlasify<B::AtlasValue> + AtlasifyMut<B::AtlasValue> + Boxify + Sizable + Send + Sync + 'static + Default,
{
  file_format: PhantomData<F>,
  baker: PhantomData<B>,
  chunk: PhantomData<C>,
}

impl<F, B, C> AssetLoader for GaikuAssetLoader<F, B, C>
where
  F: FileFormat + Send + Sync + 'static + Default,
  B: Baker + Send + Sync + 'static + Default,
  C: Chunkify<F::Value> + ChunkifyMut<F::Value> + Atlasify<B::AtlasValue> + AtlasifyMut<B::AtlasValue> + Boxify + Sizable + Send + Sync + 'static + Default,
{
  fn load<'a>(
    &'a self,
    bytes: &'a [u8],
    load_context: &'a mut LoadContext,
  ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
    Box::pin(async move {
      let mut world = World::default();

      let (chunks, atlas) = F::load::<C, GaikuTexture>(bytes.to_vec())?;

      let texture_label = "test_texture.png";

      let loaded_asset = if let Some(atlas) = &atlas {
        atlas
          .get_texture()
          .write_to_file("output/terrain_texture.png")?;
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
          base_color: Color::WHITE,
          base_color_texture: Some(
            load_context.get_handle(AssetPath::new_ref(load_context.path(), Some(texture_label))),
          ),
          ..Default::default()
        }),
      );

      let baker_options = BakerOptions {
        texture: atlas,
        ..Default::default()
      };

      world.spawn_batch(chunks.iter().map(|chunk| {
        let mesh = B::bake::<C, GaikuTexture, GaikuMesh>(chunk, &baker_options)
          .expect("Expected mesh to be baked");

        if let Some(mesh) = mesh {
          let mesh: Mesh = mesh.into();

          let name = format!("Chunk{:?}", chunk.position());
          load_context.set_labeled_asset(&name, LoadedAsset::new(mesh));
        }

        let name = format!("Chunk{:?}", &chunk.position());
        let mesh_asset_path = AssetPath::new_ref(load_context.path(), Some(&name));
        let material_asset_path = AssetPath::new_ref(load_context.path(), Some(material_label));

        PbrBundle {
          mesh: load_context.get_handle(mesh_asset_path),
          material: load_context.get_handle(material_asset_path),
          transform: Transform::from_translation(chunk.position().into()),
          ..Default::default()
        }
      }));

      load_context.set_default_asset(LoadedAsset::new(Scene::new(world)));

      Ok(())
    })
  }

  fn extensions(&self) -> &[&str] {
    &["gox"]
  }
}
