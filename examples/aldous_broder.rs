extern crate meanderer;
extern crate rand;


use meanderer::data::Grid;
use rand::Rng;
use std::collections::HashMap;


fn aldous_broder(grid: &mut Grid) {
    let mut links = Vec::new();
    let mut linked = HashMap::new();
    let mut rng = rand::thread_rng();

    if let Some(ref starting_cell) = rng.choose(&grid.cells) {
        let mut pos = starting_cell.pos.clone();
        linked.insert(pos.clone(), true);

        let mut unvisited = grid.width * grid.height - 1;

        while unvisited > 0 {
            if let Some(neighbor_pos) = rng.choose(&grid.neighbors(pos.row, pos.col)) {
                if !linked.contains_key(neighbor_pos) {
                    links.push((pos.clone(), neighbor_pos.clone()));
                    linked.insert(neighbor_pos.clone(), true);
                    unvisited -= 1;
                }

                pos = neighbor_pos.clone();
            }

        }
    }

    links.iter().map(|(p1, p2)| grid.link(p1, p2)).collect::<()>();
}

fn main() {
    let mut grid = Grid::new(6, 6);
    aldous_broder(&mut grid);
    println!("{}", grid.to_string(false));
}
