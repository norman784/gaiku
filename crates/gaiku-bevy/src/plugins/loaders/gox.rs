use anyhow;
use bevy::{
  asset::AssetLoader,
  prelude::*,
  render::{mesh::VertexAttribute, pipeline::PrimitiveTopology},
};
use gaiku::{
  bakers::{HeightMap, MarchingCubes, Voxel},
  common::{self, Baker, FileFormat},
  formats::{Gox, Png},
};
use std::path::Path;

// ==============
// LOADERS
// ==============

#[derive(Default)]
pub struct GoxLoader;

impl AssetLoader<Mesh> for GoxLoader {
  fn from_bytes(&self, asset_path: &Path, bytes: Vec<u8>) -> Result<Mesh, anyhow::Error> {
    let chunks = Gox::read(asset_path.to_str().unwrap());
    println!("asset_path: {:?} chunks: {:?}", asset_path, chunks.len());
    for chunk in chunks.iter() {
      let mesh = MarchingCubes::bake(chunk);
      if let Some(mesh) = mesh {
        return Ok(Mesh {
          primitive_topology: PrimitiveTopology::TriangleList,
          attributes: vec![
            VertexAttribute::position(mesh.vertices),
            VertexAttribute::normal(mesh.normals),
            VertexAttribute::uv(mesh.uv),
          ],
          indices: Some(mesh.indices),
        });
      }
    }

    Ok(Mesh::from(shape::Plane { size: 10.0 }))
  }

  fn extensions(&self) -> &[&str] {
    println!("registering extension");
    static EXTENSIONS: &[&str] = &["gox"];
    EXTENSIONS
  }
}

// ==============
// PLUGINS
// ==============

#[derive(Default)]
pub struct GoxPlugin;

impl Plugin for GoxPlugin {
  fn build(&self, app: &mut AppBuilder) {
    app.add_asset_loader::<Mesh, GoxLoader>();
  }
}
