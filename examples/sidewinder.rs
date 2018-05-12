extern crate meanderer;


use meanderer::algorithms::sidewinder;
use meanderer::data::Grid;


fn main() {
    let mut grid = Grid::new(6, 6);
    sidewinder(&mut grid);
    println!("{}", grid.to_string(false));
}
