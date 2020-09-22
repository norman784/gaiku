use anyhow;
use bevy::{
  asset::AssetLoader,
  prelude::*,
  render::{mesh::VertexAttribute, pipeline::PrimitiveTopology},
};
use gaiku::{
  bakers::Voxel,
  common::{self, Baker, FileFormat},
  formats::Gox,
};
use std::path::Path;

// ==============
// LOADERS
// ==============

#[derive(Default)]
pub struct GoxLoader;

impl AssetLoader<Mesh> for GoxLoader {
  fn from_bytes(&self, asset_path: &Path, bytes: Vec<u8>) -> Result<Mesh, anyhow::Error> {
    let chunks = Gox::from_bytes(bytes)?;
    let mut vertices = vec![];
    let mut normals = vec![];
    let mut uv = vec![];
    let mut indices = vec![];

    for chunk in chunks.iter() {
      if let Some(mesh) = Voxel::bake(chunk) {
        // FIXME: When/If bevy supports loading multiple meshes from the same file
        let converted_vertices = move_vertices_to_position(&mesh.vertices, chunk.position());
        let converted_indices = shift_indices(&mesh.indices, indices.len() as u32);

        vertices.extend_from_slice(&converted_vertices);
        normals.extend_from_slice(&mesh.normals);
        uv.extend_from_slice(&mesh.uv);
        indices.extend_from_slice(&converted_indices);
      }
    }

    Ok(Mesh {
      primitive_topology: PrimitiveTopology::TriangleList,
      attributes: vec![
        VertexAttribute::position(vertices),
        VertexAttribute::normal(normals),
        VertexAttribute::uv(uv),
      ],
      indices: Some(indices),
    })
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

// ==============
// UTILS
// ==============

fn move_vertices_to_position(
  vertices: &Vec<common::Vector3>,
  position: common::Vector3,
) -> Vec<common::Vector3> {
  let [cx, cy, cz] = position;
  vertices
    .iter()
    .map(|[x, y, z]| [x + cx, y + cy, z + cz])
    .collect::<Vec<[f32; 3]>>()
}

fn shift_indices(indices: &Vec<u32>, shift_value: u32) -> Vec<u32> {
  indices
    .iter()
    .map(|i| i + shift_value)
    .collect::<Vec<u32>>()
}
