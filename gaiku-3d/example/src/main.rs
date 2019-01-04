extern crate gaiku_3d;

use gaiku_3d::{
    common::{
        Baker,
        Fileformat,
    },
    bakers::VoxelBaker,
    formats::GoxReader,
};

mod exporter;

use crate::exporter::export;

fn read(name: &str) -> std::io::Result<()> {
    let file = format!("{}/assets/{}.gox", env!("CARGO_MANIFEST_DIR"), name);
    let chunks= GoxReader::read(&file);
    let mut meshes = vec![];

    for chunk in chunks {
        let mesh  = VoxelBaker::bake(&chunk);
        meshes.push((mesh, chunk.get_position()));
    }

    export(meshes, name);

    Ok(())
}

fn main() -> std::io::Result<()> {
    let _ = read("terrain");
    let _ = read("planet");

    Ok(())
}