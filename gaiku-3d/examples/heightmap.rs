use std::time::Instant;

use gaiku_3d::{
    bakers::HeightMapBaker,
    common::{Baker, FileFormat},
    formats::PNGReader,
};

mod common;

use crate::common::export;

fn read(name: &str) -> std::io::Result<()> {
    let now = Instant::now();
    let file = format!(
        "{}/examples/assets/{}.png",
        env!("CARGO_MANIFEST_DIR"),
        name
    );
    let chunks = PNGReader::read(&file);
    let mut meshes = vec![];

    let reader_elapsed = now.elapsed().as_secs();
    let now = Instant::now();

    for chunk in chunks.iter() {
        let mesh = HeightMapBaker::bake(chunk);
        if let Some(mesh) = mesh {
            meshes.push((mesh, chunk.get_position()));
        }
    }

    let baker_elapsed = now.elapsed().as_secs();
    let now = Instant::now();

    export(meshes, &format!("{}_png", name));

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
    let _ = read("heightmap");

    Ok(())
}
