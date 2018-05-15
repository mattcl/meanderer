extern crate meanderer;

use meanderer::algorithms::binary;
use meanderer::data::Grid;

fn main() {
    let mut grid = Grid::new(6, 6);
    binary(&mut grid);
    println!("{}", grid.to_string(false));
}
