use mint::{Vector2, Vector3, Vector4};
use std::collections::HashMap;

#[derive(Clone, Debug, TypedBuilder)]
pub struct Mesh {
    pub indices: Vec<u16>,
    pub vertices: Vec<Vector3<f32>>,
    #[builder(default)]
    pub normals: Vec<Vector3<f32>>,
    #[builder(default)]
    pub colors: Vec<Vector4<u8>>,
    #[builder(default)]
    pub uv: Vec<Vector2<f32>>,
    #[builder(default)]
    pub tangents: Vec<Vector4<f32>>,
}

impl Mesh {
    /// This will generate a texture from the
    /// mesh vertex colors and update the UV map
    /// Assumes a face has only one color.
    pub fn generate_texture(&mut self, width: usize, height: usize) -> Vec<u32> {
        let mut colors: HashMap<(u8, u8, u8, u8), Vec<[u16; 3]>> = HashMap::new();
        self.uv = vec![Vector2 { x: 0., y: 0. }; self.vertices.len()];
        for face in self.indices.chunks(3) {
            let (f1, f2, f3) = (face[0], face[1], face[2]);
            let color = self.colors[f1 as usize];
            let verts_for_color = colors
                .entry((color.x, color.y, color.z, color.w))
                .or_insert(vec![]);
            verts_for_color.push([f1, f2, f3]);
        }
        let current_num = colors.len();
        let colors_in_x = ((current_num as f32).sqrt()) as usize;
        let colors_in_y = current_num / colors_in_x;
        let scale_x = (colors_in_x + 1) as f32;
        let scale_y = (colors_in_y + 1) as f32;

        let mut i = 0;
        let mut result: Vec<u32> = vec![0; width * height];
        for (color, faces) in colors {
            let x_pos = (i % colors_in_x) as f32;
            let y_pos = (i / colors_in_x) as f32;
            for face in faces {
                let (i1, i2, i3) = (face[0], face[1], face[2]);
                self.uv[i1 as usize] = Vector2 {
                    x: (x_pos + 0.01) / scale_x,
                    y: (y_pos + 0.01) / scale_y,
                };
                self.uv[i2 as usize] = Vector2 {
                    x: (x_pos + 1.0 - 0.01) / scale_x,
                    y: (y_pos + 0.01) / scale_y,
                };
                self.uv[i3 as usize] = Vector2 {
                    x: (x_pos + 1.0 - 0.01) / scale_x,
                    y: (y_pos + 1.0 - 0.01) / scale_y,
                };
            }
            let y_start_tex = (y_pos / scale_y * (height as f32)) as usize;
            let y_end_tex = ((y_pos + 1.) / scale_y * (height as f32)) as usize;
            let x_start_tex = (x_pos / scale_x * (width as f32)) as usize;
            let x_end_tex = ((x_pos + 1.) / scale_x * (width as f32)) as usize;
            for y in y_start_tex..y_end_tex {
                for x in x_start_tex..x_end_tex {
                    let i = y * width + x;
                    let (r, g, b, a) = color;
                    result[i] = u32::from_le_bytes([r, g, b, a]);
                }
            }
            i += 1;
        }
        return result;
    }
}
