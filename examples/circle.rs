extern crate meanderer;

use meanderer::algorithms::iterative_backtracker;
use meanderer::data::{PolarGrid, Position};
use meanderer::rendering::{default_color_fn, polar_png, StyleBuilder};
use meanderer::solver::{dijkstra, furthest_on_rim, solve};

fn main() {
    let rows = 20;
    let mut grid = PolarGrid::new(rows);

    iterative_backtracker(&mut grid);

    let start = Position::new(0, 0);
    let end = furthest_on_rim(&mut grid, &start);
    solve(&mut grid, &start, &end);

    polar_png(
        &grid,
        &StyleBuilder::new()
            .color_fn(default_color_fn)
            .draw_solution()
            .build(),
        "circle.png",
    )
}
