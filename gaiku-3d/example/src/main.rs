extern crate gaiku_3d;
extern crate rendy;
extern crate gfx_hal;

use gaiku_3d::{
    common::{
        Baker,
        Fileformat,
        Mesh,
    },
    bakers::VoxelBaker,
    formats::GoxReader,
};

use std::{
    env,
};

mod renderer;

use crate::renderer::draw;

fn main() -> std::io::Result<()> {
    let mut path = env::current_dir()?;
    path.push("assets");
    path.push("terrain.gox");

    if let Some(file) = path.to_str() {
        let chunks= GoxReader::read(file);
        let mut meshes: Vec<Mesh> = vec![];

        for chunk in chunks {
            meshes.push(VoxelBaker::bake(&chunk));
        }

        draw(meshes);
    }

    Ok(())
}