extern crate gaiku_3d;

use std::time::Instant;

use gaiku_3d::{
    common::{
        Baker,
        FileFormat,
    },
    bakers::MarchingCubesBaker,
    formats::GoxReader,
};

mod common;

use crate::common::export;

fn read(name: &str) -> std::io::Result<()> {
    let now = Instant::now();
    let file = format!("{}/examples/assets/{}.gox", env!("CARGO_MANIFEST_DIR"), name);
    let chunks = GoxReader::read(&file);
    let mut meshes = vec![];

    let reader_elapsed = now.elapsed().as_secs();
    let now = Instant::now();

    for chunk in chunks.iter() {
        let mesh  = MarchingCubesBaker::bake(chunk);
        if let Some(mesh) = mesh {
            meshes.push((mesh, chunk.get_position()));
        }
    }

    let baker_elapsed = now.elapsed().as_secs();
    let now = Instant::now();

    export(meshes, &format!("{}_mc", name));

    println!(
        "<<{}>> Chunks: {} Reader: {} Baker: {} secs Export: {} secs",
        name,
        chunks.len(),
        reader_elapsed,
        baker_elapsed,
        now.elapsed().as_secs()
    );

    Ok(())
}

fn main() -> std::io::Result<()> {
    let _ = read("small_tree");
    let _ = read("terrain");
    let _ = read("planet");

    Ok(())
}
