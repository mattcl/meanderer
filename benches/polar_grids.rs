#[macro_use]
extern crate criterion;
extern crate meanderer;

use criterion::{Criterion, ParameterizedBenchmark};
use meanderer::algorithms::{aldous_broder, growing_tree, hunt_and_kill, iterative_backtracker, last_selection, mixed_selection, random_selection, recursive_backtracker, simplified_prims, true_prims, wilsons};
use meanderer::data::PolarGrid;

fn criterion_benchmark(c: &mut Criterion) {
    let parameters = vec![10, 20, 30, 40];
    let polar_benchmark = ParameterizedBenchmark::new(
        "aldous-broder",
        |b, i| {
            let mut grid = PolarGrid::new(*i);
            b.iter(|| aldous_broder(&mut grid))
        },
        parameters,
    )
        .with_function("wilsons", |b, i| {
            let mut grid = PolarGrid::new(*i);
            b.iter(|| wilsons(&mut grid))
        })
        .with_function("hunt-and-kill", |b, i| {
            let mut grid = PolarGrid::new(*i);
            b.iter(|| hunt_and_kill(&mut grid))
        })
        .with_function("recursive-backtracker", |b, i| {
            let mut grid = PolarGrid::new(*i);
            b.iter(|| recursive_backtracker(&mut grid))
        })
        .with_function("iterative-backtracker", |b, i| {
            let mut grid = PolarGrid::new(*i);
            b.iter(|| iterative_backtracker(&mut grid))
        })
        .with_function("simplified-prims", |b, i| {
            let mut grid = PolarGrid::new(*i);
            b.iter(|| simplified_prims(&mut grid))
        })
        .with_function("true-prims", |b, i| {
            let mut grid = PolarGrid::new(*i);
            b.iter(|| true_prims(&mut grid))
        })
        .with_function("growing-tree (last)", |b, i| {
            let mut grid = PolarGrid::new(*i);
            b.iter(|| growing_tree(&mut grid, last_selection::<PolarGrid>))
        })
        .with_function("growing-tree (random)", |b, i| {
            let mut grid = PolarGrid::new(*i);
            b.iter(|| growing_tree(&mut grid, random_selection::<PolarGrid>))
        })
        .with_function("growing-tree (mixed)", |b, i| {
            let mut grid = PolarGrid::new(*i);
            b.iter(|| growing_tree(&mut grid, mixed_selection::<PolarGrid>))
        });

    c.bench("Maze algorithms for N-row polar grids", polar_benchmark);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
