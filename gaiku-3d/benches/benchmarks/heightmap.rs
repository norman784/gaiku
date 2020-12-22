use criterion::{criterion_group, Criterion};
use gaiku_3d::{bakers::HeightMapBaker, common::{Baker, Chunk, FileFormat}, formats::GoxReader};

fn get_chunks(name: &str) -> Vec<Chunk> {
    let file = format!(
        "{}/examples/assets/{}.gox",
        env!("CARGO_MANIFEST_DIR"),
        name
    );
    
    GoxReader::read(&file)
}

fn heightmap_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("HeightMap");
    let chunks = get_chunks("terrain");

    group.bench_function("Terrain", |b| b.iter(|| {
        let mut meshes = vec![];

        for chunk in chunks.iter() {
            let mesh = HeightMapBaker::bake(chunk);
            if let Some(mesh) = mesh {
                meshes.push((mesh, chunk.position()));
            }
        }
    }));

    let chunks = get_chunks("planet");

    group.bench_function("Planet", |b| b.iter(|| {
        let mut meshes = vec![];

        for chunk in chunks.iter() {
            let mesh = HeightMapBaker::bake(chunk);
            if let Some(mesh) = mesh {
                meshes.push((mesh, chunk.position()));
            }
        }
    }));

    let chunks = get_chunks("small_tree");

    group.bench_function("Small tree", |b| b.iter(|| {
        let mut meshes = vec![];

        for chunk in chunks.iter() {
            let mesh = HeightMapBaker::bake(chunk);
            if let Some(mesh) = mesh {
                meshes.push((mesh, chunk.position()));
            }
        }
    }));

    group.finish();
}

criterion_group!(benches, heightmap_benchmark);