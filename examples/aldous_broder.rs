extern crate meanderer;


use meanderer::algorithms::aldous_broder;
use meanderer::data::Grid;


fn main() {
    let mut grid = Grid::new(6, 6);
    aldous_broder(&mut grid);
    println!("{}", grid.to_string(false));
}
