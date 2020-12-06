use gaiku_common::Mesh;

use amethyst::{
    renderer::rendy::{
        hal::Primitive,
        mesh::{Color, MeshBuilder, Normal, Position, TexCoord},
    },
    renderer::types::MeshData,
};

pub fn to_amethyst_mesh(mesh: Mesh) -> MeshData {
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
