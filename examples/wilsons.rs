extern crate meanderer;


use meanderer::algorithms::wilsons;
use meanderer::data::{Grid, Position};
use meanderer::rendering::{default_color_fn, png, StyleBuilder};
use meanderer::solver::{dijkstra, solve_to};


fn main() {
    let width = 20;
    let height = 20;
    let mut grid = Grid::new(width, height);
    wilsons(&mut grid);
    dijkstra(&mut grid);
    solve_to(&mut grid, &Position::new(height - 1, width - 1));
    println!("{}", grid.to_string(false));
    png(
        &grid,
        &StyleBuilder::new().color_fn(default_color_fn).draw_solution().build(),
        "wilsons.png"
    )
}
