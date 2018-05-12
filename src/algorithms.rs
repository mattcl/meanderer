use data::{Grid, Position};
use rand;
use rand::Rng;
use std::collections::HashMap;


pub fn binary(grid: &mut Grid) {
    let mut links = Vec::new();
    let mut rng = rand::thread_rng();

    for c in &grid.cells {
        let choices: Vec<Position> = vec![
            grid.get_pos(c.pos.row + 1, c.pos.col), // south
            grid.get_pos(c.pos.row, c.pos.col + 1)  // east
        ].iter().filter(|x| x.is_some()).map(|x| x.clone().unwrap()).collect();

        if let Some(pos) = rng.choose(&choices) {
            links.push((c.pos.clone(), pos.clone()));
        }
    }

    links.iter().map(|(p1, p2)| grid.link(p1, p2)).collect::<()>();
}

pub fn sidewinder(grid: &mut Grid) {
    let mut links = Vec::new();

    for row in 0..grid.height {
        let mut run = Vec::new();
        let mut rng = rand::thread_rng();

        for col in 0..grid.width {
            let cell = grid.get(row, col).unwrap(); // this is safe
            run.push(cell);

            let east_bound = cell.east.is_none();
            let south_bound = cell.south.is_none();

            let close = east_bound || (!south_bound && rng.gen_range(0, 3) == 0);

            if close {
                if let Some(choice) = rng.choose(&run) {
                    if let Some(ref pos) = choice.south {
                        links.push((choice.pos.clone(), pos.clone()));
                    }
                }
                run.clear();
            } else if let Some(ref pos) = cell.east {
                links.push((cell.pos.clone(), pos.clone()));
            }

        }
    }
    links.iter().map(|(p1, p2)| grid.link(p1, p2)).collect::<()>();
}

pub fn aldous_broder(grid: &mut Grid) {
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
