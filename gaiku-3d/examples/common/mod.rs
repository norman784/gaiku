use obj_exporter::{Geometry, ObjSet, Object, Primitive, Shape, TVertex, Vertex};

use gaiku_3d::common::prelude::*;

pub fn to_obj(mesh: &impl Meshify, [pos_x, pos_y, pos_z]: [f32; 3], name: &str) -> Object {
  let mut vertices = vec![];
  let mut normals = vec![];
  let mut tex_vertices = vec![];
  let mut indices = vec![];

  mesh.get_positions().iter().for_each(|[x, y, z]| {
    vertices.push(Vertex {
      x: (x + pos_x) as f64,
      y: (y + pos_y) as f64,
      z: (z + pos_z) as f64,
    })
  });

  mesh.get_normals().iter().for_each(|[x, y, z]| {
    normals.push(Vertex {
      x: *x as f64,
      y: *y as f64,
      z: *z as f64,
    })
  });

  mesh.get_uvs().iter().for_each(|[u, v]| {
    tex_vertices.push(TVertex {
      u: *u as f64,
      v: *v as f64,
      w: 0.0,
    })
  });

  for values in mesh.get_indices().chunks(3) {
    indices.push((values[0] as usize, values[1] as usize, values[2] as usize));
  }

  Object {
    name: name.to_owned(),
    vertices,
    tex_vertices,
    normals,
    geometry: vec![Geometry {
      material_name: None,
      shapes: indices
        .into_iter()
        .map(|(x, y, z)| Shape {
          primitive: Primitive::Triangle(
            (x, Some(x), Some(x)),
            (y, Some(y), Some(y)),
            (z, Some(z), Some(z)),
          ),
          groups: vec![],
          smoothing_groups: vec![],
        })
        .collect(),
    }],
  }
}

pub fn export(data: Vec<(impl Meshify, [f32; 3])>, name: &str) {
  assert!(!data.is_empty());

  let mut objects = vec![];

  for (index, (mesh, position)) in data.iter().enumerate() {
    let obj = to_obj(mesh, *position, &format!("mesh_{}", index));
    objects.push(obj);
  }

  let set = ObjSet {
    material_library: None,
    objects,
  };

  obj_exporter::export_to_file(
    &set,
    format!(
      "{}/examples/output/{}.obj",
      env!["CARGO_MANIFEST_DIR"],
      name
    ),
  )
  .unwrap();
}
