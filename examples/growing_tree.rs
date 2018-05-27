extern crate meanderer;

use meanderer::algorithms::{growing_tree, last_selection, mixed_selection, random_selection};
use meanderer::data::{Grid, Position};
use meanderer::rendering::{default_color_fn, png, StyleBuilder};
use meanderer::solver::{dijkstra, furthest_corners, solve};

fn main() {
    let width = 50;
    let height = 50;
    let mut grid = Grid::new(width, height);
    growing_tree(&mut grid, last_selection::<Grid>);
    // growing_tree(&mut grid, random_selection::<Grid>);
    // growing_tree(&mut grid, mixed_selection::<Grid>);

    let (start, end) = furthest_corners(&mut grid);
    solve(&mut grid, &start, &end);

    dijkstra(&mut grid, &Position::new((height - 1) / 2, (width - 1) / 2));
    png(
        &grid,
        &StyleBuilder::new()
            .color_fn(default_color_fn)
            .draw_solution()
            .build(),
        "growing_tree.png",
    )
}
