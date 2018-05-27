#[macro_use]
extern crate criterion;
extern crate meanderer;

use criterion::{Criterion, ParameterizedBenchmark};
use meanderer::algorithms::{aldous_broder, binary, growing_tree, hunt_and_kill, iterative_backtracker, last_selection, mixed_selection, random_selection, recursive_backtracker, sidewinder, simplified_prims, true_prims, wilsons};
use meanderer::data::Grid;

fn criterion_benchmark(c: &mut Criterion) {
    let parameters = vec![10, 20, 30, 40];
    let square_benchmark = ParameterizedBenchmark::new(
        "binary",
        |b, i| {
            let mut grid = Grid::new(*i, *i);
            b.iter(|| binary(&mut grid))
        },
        parameters,
    ).with_function("sidewinder", |b, i| {
        let mut grid = Grid::new(*i, *i);
        b.iter(|| sidewinder(&mut grid))
    })
        .with_function("aldous-broder", |b, i| {
            let mut grid = Grid::new(*i, *i);
            b.iter(|| aldous_broder(&mut grid))
        })
        .with_function("wilsons", |b, i| {
            let mut grid = Grid::new(*i, *i);
            b.iter(|| wilsons(&mut grid))
        })
        .with_function("hunt-and-kill", |b, i| {
            let mut grid = Grid::new(*i, *i);
            b.iter(|| hunt_and_kill(&mut grid))
        })
        .with_function("recursive-backtracker", |b, i| {
            let mut grid = Grid::new(*i, *i);
            b.iter(|| recursive_backtracker(&mut grid))
        })
        .with_function("iterative-backtracker", |b, i| {
            let mut grid = Grid::new(*i, *i);
            b.iter(|| iterative_backtracker(&mut grid))
        })
        .with_function("simplified-prims", |b, i| {
            let mut grid = Grid::new(*i, *i);
            b.iter(|| simplified_prims(&mut grid))
        })
        .with_function("true-prims", |b, i| {
            let mut grid = Grid::new(*i, *i);
            b.iter(|| true_prims(&mut grid))
        })
        .with_function("growing-tree (last)", |b, i| {
            let mut grid = Grid::new(*i, *i);
            b.iter(|| growing_tree(&mut grid, last_selection::<Grid>))
        })
        .with_function("growing-tree (random)", |b, i| {
            let mut grid = Grid::new(*i, *i);
            b.iter(|| growing_tree(&mut grid, random_selection::<Grid>))
        })
        .with_function("growing-tree (mixed)", |b, i| {
            let mut grid = Grid::new(*i, *i);
            b.iter(|| growing_tree(&mut grid, mixed_selection::<Grid>))
        });

    c.bench("Maze algorithms for NxN grids", square_benchmark);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
