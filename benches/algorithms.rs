#[macro_use]
extern crate criterion;
extern crate meanderer;


use criterion::{Criterion, ParameterizedBenchmark};
use meanderer::algorithms::{aldous_broder, binary, sidewinder, wilsons};
use meanderer::data::Grid;



fn criterion_benchmark(c: &mut Criterion) {
    let parameters = vec![10, 20, 30, 40];
    let mut benchmark = ParameterizedBenchmark::new(
        "binary",
        |b, i| {
            let mut grid = Grid::new(*i, *i);
            b.iter(|| binary(&mut grid))
        },
        parameters
    ).with_function(
        "sidewinder",
        |b, i| {
            let mut grid = Grid::new(*i, *i);
            b.iter(|| sidewinder(&mut grid))
        }
    ).with_function(
        "aldous-broder",
        |b, i| {
            let mut grid = Grid::new(*i, *i);
            b.iter(|| aldous_broder(&mut grid))
        }
    ).with_function(
        "wilsons",
        |b, i| {
            let mut grid = Grid::new(*i, *i);
            b.iter(|| wilsons(&mut grid))
        }
    );

    c.bench("Maze algorithms for NxN grids", benchmark);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
