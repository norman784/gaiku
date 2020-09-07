use obj_exporter::{Geometry, ObjSet, Object, Primitive, Shape, Vertex};

use gaiku::common::{Mesh, Vector3};

pub fn to_obj(mesh: &Mesh, position: &Vector3, name: &str) -> Object {
  let mut vertices = vec![];
  let mut indices = vec![];

  for vertex in mesh.vertices.iter() {
    let x = vertex[0] as f64 + position[0] as f64;
    let y = vertex[1] as f64 + position[1] as f64;
    let z = vertex[2] as f64 + position[2] as f64;
    vertices.push((x, y, z));
  }

  for i in (0..mesh.indices.len()).step_by(3) {
    indices.push((mesh.indices[i], mesh.indices[i + 1], mesh.indices[i + 2]))
  }

  Object {
    name: name.to_owned(),
    vertices: vertices
      .into_iter()
      .map(|(x, y, z)| Vertex { x, y, z })
      .collect(),
    tex_vertices: vec![],
    normals: vec![],
    geometry: vec![Geometry {
      material_name: None,
      shapes: indices
        .into_iter()
        .map(|(x, y, z)| Shape {
          primitive: Primitive::Triangle(
            (x as usize, None, None),
            (y as usize, None, None),
            (z as usize, None, None),
          ),
          groups: vec![],
          smoothing_groups: vec![],
        })
        .collect(),
    }],
  }
}

pub fn export(data: Vec<(Mesh, Vector3)>, name: &str) {
  let mut objects = vec![];

  for (index, (mesh, position)) in data.iter().enumerate() {
    let obj = to_obj(mesh, position, &format!("mesh_{}", index));
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
