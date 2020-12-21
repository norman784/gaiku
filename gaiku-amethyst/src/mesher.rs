use gaiku_common::Mesh;

use amethyst::{
  renderer::palette::Srgba,
  renderer::rendy::{
    hal::image::{Filter, Kind, SamplerInfo, ViewKind, WrapMode},
    hal::Primitive,
    mesh::{Color, MeshBuilder, Normal, Position, TexCoord},
    texture::{pixel::Rgba8Srgb, TextureBuilder},
  },
  renderer::types::{MeshData, TextureData},
};

/// Creates an ametheyst texture data which can be attached to
/// material to give the mesh color. Texture size is fixed at 1024x1024
pub fn get_amethyst_texture(mesh: &mut Mesh, width: u32, height: u32) -> TextureData {
  let tex_data = mesh.generate_texture(width as usize, height as usize);
  let pixel_data = tex_data
    .into_iter()
    .map(|rgba| {
      let [red, green, blue, alpha] = rgba.to_le_bytes();
      Rgba8Srgb::from(Srgba::new(red, green, blue, alpha))
    })
    .collect::<Vec<Rgba8Srgb>>();
  let texture_builder = TextureBuilder::new()
    .with_kind(Kind::D2(width as u32, height as u32, 1, 1))
    .with_view_kind(ViewKind::D2)
    .with_data_width(width as u32)
    .with_data_height(height as u32)
    .with_sampler_info(SamplerInfo::new(Filter::Linear, WrapMode::Clamp))
    .with_data(pixel_data);
  return texture_builder.into();
}

/// Creates an ametheyst mesh data which can be attached to
/// entity to render the mesh
pub fn to_amethyst_mesh(mesh: &Mesh) -> MeshData {
  let mut vertices: Vec<Position> = vec![];
  let mut colors: Vec<Color> = vec![];
  let mut normals: Vec<Normal> = vec![];
  let mut tex_cooor: Vec<TexCoord> = vec![];
  let mut indices = vec![];
  for vertex in mesh.vertices.iter() {
    let x = vertex.x;
    let y = vertex.y;
    let z = vertex.z;
    vertices.push([x, y, z].into());
    tex_cooor.push([0., 0.].into()); // TODO: Add read tex coordinates and colors
  }

  for i in 0..mesh.vertices.len() {
    let (x, y, z) = if i < mesh.normals.len() {
      (mesh.normals[i].x, mesh.normals[i].y, mesh.normals[i].z)
    } else {
      (0., 0., 0.)
    };
    normals.push([x, y, z].into());
  }
  for i in 0..mesh.vertices.len() {
    let (r, g, b, a) = if i < mesh.colors.len() {
      (
        mesh.colors[i].x as f32 / 255.,
        mesh.colors[i].y as f32 / 255.,
        mesh.colors[i].z as f32 / 255.,
        mesh.colors[i].w as f32 / 255.,
      )
    } else {
      (1., 1., 1., 1.)
    };
    colors.push([r, g, b, a].into());
  }
  for i in (0..mesh.indices.len()).step_by(3) {
    indices.push(mesh.indices[i] as u32);
    indices.push(mesh.indices[i + 1] as u32);
    indices.push(mesh.indices[i + 2] as u32);
  }

  let ame = MeshBuilder::new()
    .with_vertices(vertices)
    .with_vertices(colors)
    .with_vertices(tex_cooor)
    .with_vertices(normals)
    .with_indices(indices)
    .with_prim_type(Primitive::TriangleList);

  ame.into()
}

/// Creates both texture and mesh data for amethyst
pub fn to_amethyst_mesh_ww_tex(
  mesh: &mut Mesh,
  width: u32,
  height: u32,
) -> (MeshData, TextureData) {
  let tex_data = get_amethyst_texture(mesh, width, height);
  return (to_amethyst_mesh(mesh), tex_data);
}
