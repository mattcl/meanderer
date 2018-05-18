extern crate meanderer;

use meanderer::algorithms::iterative_backtracker;
use meanderer::data::{PolarGrid, Position};
use meanderer::rendering::{default_color_fn, polar_png, StyleBuilder};
use meanderer::solver::{dijkstra, solve};

fn main() {
    let rows = 30;
    let mut grid = PolarGrid::new(rows);
    iterative_backtracker(&mut grid);
    // solve(&mut grid, &start, &end); TODO

    // we do this again afterward to produce a weight map from the center of the maze
    // since it makes the rendering nicer
    dijkstra(&mut grid, &Position::new(0, 0));
    polar_png(
        &grid,
        &StyleBuilder::new()
            .color_fn(default_color_fn)
            .draw_solution()
            .build(),
        "circle.png",
    )
}
