extern crate meanderer;
extern crate rand;


use meanderer::data::{Grid, Position};
use rand::Rng;


fn sidewinder(grid: &mut Grid) {
    // TODO
}

fn main() {
    let mut grid = Grid::new(6, 6);
    sidewinder(&mut grid);
    println!("{}", grid.to_string(false));
}
