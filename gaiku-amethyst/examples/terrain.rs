//! Gaiku Amethyst Terrain
//!
//! A small demo of gaiku-amethust
//! using a terrain goxel.

use amethyst::{
    assets::{Handle, Loader},
    controls::FlyControlBundle,
    controls::FlyControlTag,
    core::math::{Matrix4, Vector3, Vector4},
    core::transform::Transform,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        light::{DirectionalLight, Light},
        palette::rgb::Rgb,
        palette::Srgb,
        plugins::{RenderShaded3D, RenderSkybox, RenderToWindow},
        types::DefaultBackend,
        ActiveCamera, Camera, Material, MaterialDefaults, Mesh, RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

use gaiku_3d::{
    bakers::VoxelBaker,
    common::{Baker, FileFormat},
    formats::GoxReader,
};
use gaiku_amethyst::mesher::to_amethyst_mesh_ww_tex;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let assets_dir = app_root.join("examples").join("assets");

    let display_config_path = assets_dir.join("display.ron");

    let binding_path = assets_dir.join("bindings.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(binding_path)?;

    let render_bund = RenderingBundle::<DefaultBackend>::new()
        // The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
        .with_plugin(
            RenderToWindow::from_config_path(display_config_path)?.with_clear([0.0, 0.0, 0.0, 1.0]),
        )
        .with_plugin(RenderShaded3D::default())
        .with_plugin(RenderUi::default())
        .with_plugin(RenderSkybox::with_colors(
            Srgb::new(0.82, 0.51, 0.50),
            Srgb::new(0.18, 0.11, 0.85),
        ));

    let game_data = GameDataBuilder::default()
        .with_bundle(render_bund)?
        // With transform systems for position tracking
        .with_bundle(TransformBundle::new())?
        .with_bundle(FlyControlBundle::<StringBindings>::new(
            Some(String::from("right")),
            Some(String::from("up")),
            Some(String::from("forward")),
        ))?
        .with_bundle(input_bundle)?
        .with_bundle(UiBundle::<StringBindings>::new())?;

    let mut game = Application::new(assets_dir, GameLoad::new(), game_data)?;

    game.run();
    Ok(())
}

pub struct GameLoad;

impl SimpleState for GameLoad {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        self.initialise_camera(world);
        self.add_light(world);
        self.build_terrain(world);
    }
}

impl Default for GameLoad {
    fn default() -> Self {
        Self::new()
    }
}

impl GameLoad {
    pub fn new() -> Self {
        Self {}
    }

    fn initialise_camera(&self, world: &mut World) {
        let mut transform = Transform::default();
        transform.set_translation_xyz(0., 2., -10.0);
        transform.face_towards(Vector3::new(0., 0., 0.), Vector3::new(0., 1., 0.));

        let cam_ent = world
            .create_entity()
            .with(Camera::standard_3d(600., 400.))
            .with(transform)
            .with(FlyControlTag)
            .build();
        let act_cam: &mut ActiveCamera = world.get_mut().expect("There shoud be an active camera");
        act_cam.entity = Some(cam_ent);
    }

    fn add_light(&self, world: &mut World) {
        world
            .create_entity()
            .with(Light::from(DirectionalLight {
                color: Rgb::new(1.0, 1.0, 1.0),
                direction: [-1.0, -1.0, -1.0].into(),
                intensity: 1.0,
            }))
            .build();
    }

    fn build_terrain(&self, world: &mut World) {
        let file = format!(
            "{}/examples/assets/{}.gox",
            env!("CARGO_MANIFEST_DIR"),
            "terrain"
        );
        let chunks = GoxReader::read(&file);
        let mut meshes = vec![];

        for chunk in chunks.iter() {
            let mesh = VoxelBaker::bake(chunk);
            if let Some(mesh) = mesh {
                meshes.push((mesh, chunk.position()));
            }
        }

        let scale = Vector3::new(0.1, 0.1, 0.1);
        let swap_axes = true;
        let transform = if swap_axes {
            Matrix4::from_euler_angles(-90., 0., 0.)
        } else {
            Matrix4::identity()
        };
        for (mut mesh_gox, position) in meshes {
            let (mesh, mat) = {
                if swap_axes {
                    // Swap y/z for amethyst coordinate system
                    for vert in &mut mesh_gox.vertices {
                        let v = Vector4::new(vert.x, vert.y, vert.z, 1.);
                        let vtran = transform * v;
                        vert.x = vtran[0];
                        vert.y = vtran[1];
                        vert.z = vtran[2];
                    }
                    for normal in &mut mesh_gox.normals {
                        let v = Vector4::new(normal.x, normal.y, normal.z, 1.);
                        let vtran = transform * v;
                        normal.x = vtran[0];
                        normal.y = vtran[1];
                        normal.z = vtran[2];
                    }
                }
                let loader = world.read_resource::<Loader>();
                let mat_default = world.read_resource::<MaterialDefaults>();
                let (mesh_data, tex_data) = to_amethyst_mesh_ww_tex(&mut mesh_gox, 32, 32);
                let mesh: Handle<Mesh> =
                    loader.load_from_data(mesh_data, (), &world.read_resource());
                let tex = loader.load_from_data(tex_data, (), &world.read_resource());
                let mat: Handle<Material> = loader.load_from_data(
                    Material {
                        albedo: tex,
                        ..mat_default.0.clone()
                    },
                    (),
                    &world.read_resource(),
                );
                (mesh, mat)
            };
            let mut pos = Transform::default();

            let position_trans = {
                let v = Vector4::new(position.x, position.y, position.z, 1.);
                let vtrans = transform * v;
                Vector3::new(
                    vtrans[0] * scale[0],
                    vtrans[1] * scale[1],
                    vtrans[2] * scale[2],
                )
            };
            pos.set_translation(position_trans);
            pos.set_scale(scale);

            let _voxel = world.create_entity().with(mesh).with(mat).with(pos).build();
        }
    }
}
