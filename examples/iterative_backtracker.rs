extern crate meanderer;

use meanderer::algorithms::iterative_backtracker;
use meanderer::data::{Grid, Position};
use meanderer::rendering::{default_color_fn, png, StyleBuilder};
use meanderer::solver::{dijkstra, furthest_corners, solve};

fn main() {
    let width = 50;
    let height = 50;
    let mut grid = Grid::new(width, height);
    iterative_backtracker(&mut grid);
    let (start, end) = furthest_corners(&mut grid);
    solve(
        &mut grid,
        &start,
        &end,
    );

    // we do this again afterward to produce a weight map from the center of the maze
    // since it makes the rendering nicer
    dijkstra(&mut grid, &Position::new((height - 1) / 2, (width - 1) / 2));
    png(
        &grid,
        &StyleBuilder::new()
            .color_fn(default_color_fn)
            .draw_solution()
            .build(),
        "iterative_backtracker.png",
    )
}
