use gaiku_3d::common::Texture2d;

fn main() {
  let mut texture = Texture2d::new(8, 8);

  let black = [0, 0, 0, 255];
  let gray = [224, 224, 224, 255];
  let green_dark = [103, 158, 55, 255];
  let green_light = [124, 179, 66, 255];
  let orange = [255, 85, 33, 255];
  let purple = [135, 14, 78, 255];
  let red = [184, 28, 28, 255];
  let white = [255, 255, 255, 255];

  texture.set_pixel(2, 0, orange);
  texture.set_pixel(3, 0, green_light);
  texture.set_pixel(4, 0, green_dark);
  texture.set_pixel(5, 0, green_light);

  texture.set_pixel(1, 1, orange);
  texture.set_pixel(2, 1, green_light);
  texture.set_pixel(3, 1, black);
  texture.set_pixel(4, 1, green_light);
  texture.set_pixel(5, 1, green_light);
  texture.set_pixel(6, 1, green_light);
  texture.set_pixel(7, 1, green_light);

  texture.set_pixel(2, 2, white);
  texture.set_pixel(3, 2, white);
  texture.set_pixel(4, 2, green_light);
  texture.set_pixel(5, 2, green_light);
  texture.set_pixel(6, 2, green_light);
  texture.set_pixel(7, 2, green_light);

  texture.set_pixel(2, 3, orange);
  texture.set_pixel(2, 3, white);
  texture.set_pixel(3, 3, white);
  texture.set_pixel(4, 3, green_light);
  texture.set_pixel(5, 3, green_light);
  texture.set_pixel(6, 3, green_light);
  texture.set_pixel(7, 3, green_light);

  texture.set_pixel(2, 4, orange);
  texture.set_pixel(3, 4, gray);
  texture.set_pixel(4, 4, white);
  texture.set_pixel(5, 4, white);
  texture.set_pixel(6, 4, white);

  texture.set_pixel(0, 5, green_dark);
  texture.set_pixel(1, 5, orange);
  texture.set_pixel(2, 5, green_dark);
  texture.set_pixel(3, 5, green_dark);
  texture.set_pixel(4, 5, gray);
  texture.set_pixel(5, 5, gray);
  texture.set_pixel(6, 5, green_dark);

  texture.set_pixel(1, 6, green_dark);
  texture.set_pixel(2, 6, green_light);
  texture.set_pixel(3, 6, green_dark);
  texture.set_pixel(4, 6, gray);
  texture.set_pixel(5, 6, gray);
  texture.set_pixel(6, 6, green_dark);
  texture.set_pixel(7, 6, green_dark);

  texture.set_pixel(2, 7, red);
  texture.set_pixel(3, 7, red);
  texture.set_pixel(5, 7, purple);
  texture.set_pixel(6, 7, purple);

  let name = format!("{}/examples/output/texture.png", env!("CARGO_MANIFEST_DIR"),);
  texture.write_to_file(&name).unwrap();
}
