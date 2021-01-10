use obj_exporter::{Geometry, ObjSet, Object, Primitive, Shape, TVertex, Vertex};

use gaiku_3d::common::{
  mesh::{Indices, VertexAttribute, VertexAttributeValues},
  Mesh,
};

pub fn to_obj(mesh: &Mesh, [pos_x, pos_y, pos_z]: [f32; 3], name: &str) -> Object {
  let mut vertices = vec![];
  let mut normals = vec![];
  let mut tex_vertices = vec![];
  let mut indices = vec![];

  if let Some(value) = mesh.get_attributes(VertexAttribute::Position) {
    match value {
      VertexAttributeValues::Float3(arr) => arr.iter().for_each(|[x, y, z]| {
        vertices.push(Vertex {
          x: (x + pos_x) as f64,
          y: (y + pos_y) as f64,
          z: (z + pos_z) as f64,
        })
      }),
      _ => {}
    }
  }

  if let Some(value) = mesh.get_attributes(VertexAttribute::Normal) {
    match value {
      VertexAttributeValues::Float3(arr) => arr.iter().for_each(|[x, y, z]| {
        normals.push(Vertex {
          x: *x as f64,
          y: *y as f64,
          z: *z as f64,
        })
      }),
      _ => {}
    }
  }

  if let Some(value) = mesh.get_attributes(VertexAttribute::UV) {
    match value {
      VertexAttributeValues::Float2(arr) => arr.iter().for_each(|[u, v]| {
        tex_vertices.push(TVertex {
          u: *u as f64,
          v: *v as f64,
          w: 0.0,
        })
      }),
      _ => {}
    }
  }

  if let Some(values) = &mesh.indices {
    match values {
      Indices::U16(values) => {
        for i in (0..values.len()).step_by(3) {
          indices.push((
            values[i] as usize,
            values[i + 1] as usize,
            values[i + 2] as usize,
          ));
        }
      }
      Indices::U32(values) => {
        for i in (0..values.len()).step_by(3) {
          indices.push((
            values[i] as usize,
            values[i + 1] as usize,
            values[i + 2] as usize,
          ));
        }
      }
    }
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

pub fn export(data: Vec<(Mesh, [f32; 3])>, name: &str) {
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
