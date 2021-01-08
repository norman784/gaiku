use obj_exporter::{Geometry, ObjSet, Object, Primitive, Shape, TVertex, Vertex};

use gaiku_3d::common::{
  mint::{Vector2, Vector3},
  Mesh,
};

pub fn to_obj(mesh: &Mesh, position: &Vector3<f32>, name: &str) -> Object {
  let mut vertices = vec![];
  let mut indices = vec![];

  for vertex in mesh.vertices.iter() {
    let x = vertex.x as f64 + position.x as f64;
    let y = vertex.y as f64 + position.y as f64;
    let z = vertex.z as f64 + position.z as f64;
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
    tex_vertices: mesh
      .uv
      .clone()
      .into_iter()
      .map(|Vector2 { x, y }| TVertex {
        u: x as f64,
        v: y as f64,
        w: 0.0,
      })
      .collect(),
    normals: mesh
      .normals
      .clone()
      .into_iter()
      .map(|Vector3 { x, y, z }| Vertex {
        x: x as f64,
        y: y as f64,
        z: z as f64,
      })
      .collect(),
    geometry: vec![Geometry {
      material_name: None,
      shapes: indices
        .into_iter()
        .map(|(x, y, z)| Shape {
          primitive: Primitive::Triangle(
            (x as usize, Some(x as usize), Some(x as usize)),
            (y as usize, Some(y as usize), Some(y as usize)),
            (z as usize, Some(z as usize), Some(z as usize)),
          ),
          groups: vec![],
          smoothing_groups: vec![],
        })
        .collect(),
    }],
  }
}

pub fn export(data: Vec<(Mesh, Vector3<f32>)>, name: &str) {
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
