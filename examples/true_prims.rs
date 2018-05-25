extern crate meanderer;

use meanderer::algorithms::true_prims;
use meanderer::data::{Grid, Position};
use meanderer::rendering::{default_color_fn, png, StyleBuilder};
use meanderer::solver::{dijkstra, furthest_corners, solve};

fn main() {
    let width = 20;
    let height = 20;
    let mut grid = Grid::new(width, height);
    true_prims(&mut grid);

    let (start, end) = furthest_corners(&mut grid);
    solve(&mut grid, &start, &end);

    dijkstra(&mut grid, &Position::new((height - 1) / 2, (width - 1) / 2));
    png(
        &grid,
        &StyleBuilder::new()
            .color_fn(default_color_fn)
            .draw_solution()
            .build(),
        "true_prims.png",
    )
}
