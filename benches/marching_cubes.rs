use benchmarks::*;
use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    marching_cubes::benches,
}
