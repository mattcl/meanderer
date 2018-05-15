use data::{Grid, Position};
use rand;
use rand::{Rng, ThreadRng};
use std::collections::HashSet;

pub fn binary(grid: &mut Grid) {
    let mut rng = rand::thread_rng();

    for i in 0..grid.cells.len() {
        let c = grid.cells[i].pos.clone();
        let choices: Vec<Position> = vec![
            grid.get_pos(c.row + 1, c.col), // south
            grid.get_pos(c.row, c.col + 1), // east
        ].iter()
            .cloned()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();

        if let Some(pos) = rng.choose(&choices) {
            grid.link(&c, pos);
        }
    }
}

pub fn sidewinder(grid: &mut Grid) {
    let mut links = Vec::new();
    let mut rng = rand::thread_rng();

    for row in 0..grid.height {
        let mut run = Vec::new();

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
    links
        .iter()
        .map(|(p1, p2)| grid.link(p1, p2))
        .collect::<()>();
}

pub fn aldous_broder(grid: &mut Grid) {
    let mut links = Vec::new();
    let mut linked = HashSet::new();
    let mut rng = rand::thread_rng();

    if let Some(ref starting_cell) = rng.choose(&grid.cells) {
        let mut pos = starting_cell.pos.clone();
        linked.insert(pos.clone());

        let mut unvisited = grid.width * grid.height - 1;

        while unvisited > 0 {
            if let Some(neighbor_pos) = rng.choose(&grid.neighbors(pos.row, pos.col)) {
                if !linked.contains(neighbor_pos) {
                    links.push((pos.clone(), neighbor_pos.clone()));
                    linked.insert(neighbor_pos.clone());
                    unvisited -= 1;
                }

                pos = neighbor_pos.clone();
            }
        }
    }

    links
        .iter()
        .map(|(p1, p2)| grid.link(p1, p2))
        .collect::<()>();
}

pub fn wilsons(grid: &mut Grid) {
    let mut unvisited: HashSet<Position> = grid.cells.iter().map(|c| c.pos.clone()).collect();
    let mut rng = rand::thread_rng();

    _make_initial(&mut unvisited);

    while !unvisited.is_empty() {
        if let Some(start) = rng.choose(&unvisited.iter().cloned().collect::<Vec<Position>>()) {
            // walk from the start to a visisted cell
            let mut path = Vec::new();
            path.push(start.clone());
            _walk(grid, &mut path, &mut unvisited);
        }
    }
}

fn _make_initial(unvisited: &mut HashSet<Position>) -> Option<Position> {
    let mut rng = rand::thread_rng();

    let options = unvisited.iter().cloned().collect::<Vec<Position>>();

    if let Some(initial) = rng.choose(&options) {
        unvisited.remove(initial);
        return Some(initial.clone());
    }

    None
}

fn _walk(grid: &mut Grid, path: &mut Vec<Position>, unvisited: &mut HashSet<Position>) {
    let mut rng = rand::thread_rng();

    let mut path_set = path.iter().cloned().collect::<HashSet<Position>>();

    while let Some(current) = path.get(path.len() - 1).cloned() {
        let mut choices = grid.neighbors(current.row, current.col);

        // if there is a previous position in the path, we can ensure that
        // we don't waste time by randomly selecting that element
        if path.len() > 1 {
            choices.retain(|c| c != &path[path.len() - 2]);
        }

        if let Some(next) = rng.choose(&choices) {
            if !unvisited.contains(next) {
                // link everything from path to next
                path.push(next.clone());
                for i in 0..path.len() {
                    if i < path.len() - 1 {
                        grid.link(&path[i], &path[i + 1]);
                    }
                    unvisited.remove(&path[i]);
                }

                // we're done with this walk
                return;
            } else if path_set.contains(next) {
                // remove loop
                for i in 0..path.len() {
                    if path[i] == *next {
                        path.truncate(i + 1);
                        break;
                    }
                }

                // reset the path set
                path_set = path.iter().cloned().collect::<HashSet<Position>>();
            } else {
                path.push(next.clone());
                path_set.insert(next.clone());
            }
        }
    }
}

pub fn hunt_and_kill(grid: &mut Grid) {
    // pick a random cell
    let mut unvisited: HashSet<Position> = grid.cells.iter().map(|c| c.pos.clone()).collect();
    let mut rng = rand::thread_rng();

    if let Some(start) = _make_initial(&mut unvisited) {
        let mut current = start;

        // no tail recursion in rust yet :(
        loop {
            if let Some(next) = _hunt_and_kill(grid, &mut unvisited, &mut rng, &current) {
                current = next.clone()
            } else {
                break;
            }
        }
    }
}

fn _hunt_and_kill(
    grid: &mut Grid,
    unvisited: &mut HashSet<Position>,
    rng: &mut ThreadRng,
    current: &Position,
) -> Option<Position> {
    let neighbors = grid.neighbors(current.row, current.col);
    let unvisited_neighbors: Vec<Position> = neighbors
        .iter()
        .cloned()
        .filter(|x| unvisited.contains(x))
        .collect();

    let mut next: Option<Position> = None;

    if !unvisited_neighbors.is_empty() {
        if let Some(neighbor) = rng.choose(&unvisited_neighbors) {
            grid.link(current, neighbor);
            next = Some(neighbor.clone());
        }
    } else if !unvisited.is_empty() {
        for i in 0..grid.cells.len() {
            let cur = grid.cells[i].pos.clone();
            if !grid.has_links(cur.row, cur.col) {
                if let Some(linked_neighbor) = rng.choose(&grid.neighbors(cur.row, cur.col)
                    .iter()
                    .filter(|pos| grid.has_links(pos.row, pos.col))
                    .collect::<Vec<&Position>>())
                {
                    grid.link(&cur, linked_neighbor);
                    next = Some(cur.clone());
                    break;
                }
            }
        }
    }

    if let Some(ref next) = next {
        unvisited.remove(next);
    }

    next
}
