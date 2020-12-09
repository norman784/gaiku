use criterion::criterion_main;
use benchmarks::*;

mod benchmarks;

criterion_main!{
    heightmap::benches,
    marching_cubes::benches,
    voxel::benches,
}