extern crate meanderer;


use meanderer::algorithms::wilsons;
use meanderer::data::{Grid, Position};
use meanderer::rendering::{default_color_fn, png, StyleBuilder};
use meanderer::solver::{dijkstra, solve};


fn main() {
    let width = 30;
    let height = 30;
    let mut grid = Grid::new(width, height);
    wilsons(&mut grid);
    solve(&mut grid, &Position::new(0, 0), &Position::new(height - 1, width - 1));

    // we do this again afterward to produce a weight map from the center of the maze
    // since it makes the rendering nicer
    dijkstra(&mut grid, &Position::new((height - 1) / 2, (width - 1) / 2));
    println!("{}", grid.to_string(false));
    png(
        &grid,
        &StyleBuilder::new().color_fn(default_color_fn).draw_solution().build(),
        "wilsons.png"
    )
}
