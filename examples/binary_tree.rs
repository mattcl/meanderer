extern crate meanderer;
extern crate rand;


use meanderer::data::{Grid, Position};
use rand::Rng;


fn binary(grid: &mut Grid) {
    let mut links = Vec::new();

    for c in &grid.cells {
        let choices: Vec<Position> = vec![
            grid.get_pos(c.pos.row + 1, c.pos.col),
            grid.get_pos(c.pos.row, c.pos.col + 1)
        ].iter().filter(|x| x.is_some()).map(|x| x.clone().unwrap()).collect();

        if let Some(pos) = rand::thread_rng().choose(&choices) {
            links.push((c.pos.clone(), pos.clone()));
        }
    }

    links.iter().map(|(p1, p2)| grid.link(p1, p2)).collect::<()>();
}

fn main() {
    let mut grid = Grid::new(6, 6);
    binary(&mut grid);
    println!("{}", grid.to_string(false));
}
