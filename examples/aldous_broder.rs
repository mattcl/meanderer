extern crate meanderer;


use meanderer::algorithms::aldous_broder;
use meanderer::data::Grid;
use meanderer::rendering::{png, StyleBuilder};


fn main() {
    let mut grid = Grid::new(20, 20);
    aldous_broder(&mut grid);
    println!("{}", grid.to_string(false));
    png(&grid, &StyleBuilder::new().build(), "aldous-broder.png")
}
