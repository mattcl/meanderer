use data::cell::MazeCell;
use data::grid::{Grid, MazeGrid};
use data::pos::{MazePosition, Position};
use linked_hash_set::LinkedHashSet;
use rand;
use rand::{Rng, ThreadRng};
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub fn binary(grid: &mut Grid) {
    let mut rng = rand::thread_rng();

    for i in 0..grid.cells.len() {
        let c = grid.cells[i].pos.clone();
        let choices: Vec<Position> = vec![
            grid.get_pos(&Position::new(c.row + 1, c.col)), // south
            grid.get_pos(&Position::new(c.row, c.col + 1)), // east
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
            let cell = grid.get(&Position::new(row, col)).unwrap(); // this is safe
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
    for (p1, p2) in links.iter() {
        grid.link(p1, p2);
    }
}

pub fn aldous_broder<G: MazeGrid>(grid: &mut G) {
    let mut links = Vec::new();
    let mut linked = HashSet::new();
    let mut rng = rand::thread_rng();

    if let Some(ref starting_cell) = rng.choose(&grid.cells()) {
        let mut pos = starting_cell.pos().clone();
        linked.insert(pos.clone());

        let mut unvisited = grid.cells().len() - 1;

        while unvisited > 0 {
            if let Some(neighbor_pos) = rng.choose(&grid.neighbors(&pos)) {
                if !linked.contains(neighbor_pos) {
                    links.push((pos.clone(), neighbor_pos.clone()));
                    linked.insert(neighbor_pos.clone());
                    unvisited -= 1;
                }

                pos = neighbor_pos.clone();
            }
        }
    }

    for (p1, p2) in links.iter() {
        grid.link(p1, p2);
    }
}

pub fn wilsons<G: MazeGrid>(grid: &mut G) {
    let mut unvisited: HashSet<<G::CellType as MazeCell>::PositionType> =
        grid.cells().iter().map(|c| c.pos().clone()).collect();
    let mut rng = rand::thread_rng();

    _make_initial(&mut unvisited);

    while !unvisited.is_empty() {
        if let Some(start) = rng.choose(&unvisited
            .iter()
            .cloned()
            .collect::<Vec<<G::CellType as MazeCell>::PositionType>>())
        {
            // walk from the start to a visisted cell
            let mut path = Vec::new();
            path.push(start.clone());
            _walk(grid, &mut path, &mut unvisited);
        }
    }
}

fn _make_initial<P: MazePosition>(unvisited: &mut HashSet<P>) -> Option<P> {
    let mut rng = rand::thread_rng();

    let options = unvisited.iter().cloned().collect::<Vec<P>>();

    if let Some(initial) = rng.choose(&options) {
        unvisited.remove(initial);
        return Some(initial.clone());
    }

    None
}

fn _walk<G: MazeGrid>(
    grid: &mut G,
    path: &mut Vec<<G::CellType as MazeCell>::PositionType>,
    unvisited: &mut HashSet<<G::CellType as MazeCell>::PositionType>,
) {
    let mut rng = rand::thread_rng();

    let mut path_set = path.iter()
        .cloned()
        .collect::<HashSet<<G::CellType as MazeCell>::PositionType>>();

    while let Some(current) = path.get(path.len() - 1).cloned() {
        let mut choices = grid.neighbors(&current);

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
                path_set = path.iter()
                    .cloned()
                    .collect::<HashSet<<G::CellType as MazeCell>::PositionType>>();
            } else {
                path.push(next.clone());
                path_set.insert(next.clone());
            }
        }
    }
}

pub fn hunt_and_kill<G: MazeGrid>(grid: &mut G) {
    // pick a random cell
    let mut unvisited: HashSet<<G::CellType as MazeCell>::PositionType> =
        grid.cells().iter().map(|c| c.pos().clone()).collect();
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

fn _hunt_and_kill<G: MazeGrid>(
    grid: &mut G,
    unvisited: &mut HashSet<<G::CellType as MazeCell>::PositionType>,
    rng: &mut ThreadRng,
    current: &<G::CellType as MazeCell>::PositionType,
) -> Option<<G::CellType as MazeCell>::PositionType> {
    let neighbors = grid.neighbors(current);
    let unvisited_neighbors: Vec<<G::CellType as MazeCell>::PositionType> = neighbors
        .iter()
        .filter(|x| unvisited.contains(x))
        .cloned()
        .collect();

    let mut next: Option<<G::CellType as MazeCell>::PositionType> = None;

    if !unvisited_neighbors.is_empty() {
        if let Some(neighbor) = rng.choose(&unvisited_neighbors) {
            grid.link(current, neighbor);
            next = Some(neighbor.clone());
        }
    } else if !unvisited.is_empty() {
        for i in 0..grid.cells().len() {
            let cur = grid.cells()[i].pos().clone();
            if !grid.has_links(&cur) {
                if let Some(linked_neighbor) = rng.choose(&grid.neighbors(&cur)
                    .iter()
                    .filter(|pos| grid.has_links(pos))
                    .collect::<Vec<&<G::CellType as MazeCell>::PositionType>>())
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

pub fn recursive_backtracker<G: MazeGrid>(grid: &mut G) {
    let mut unvisited: HashSet<<G::CellType as MazeCell>::PositionType> =
        grid.cells().iter().map(|c| c.pos().clone()).collect();
    let mut rng = rand::thread_rng();

    if let Some(start) = _make_initial(&mut unvisited) {
        _recurse(grid, &mut unvisited, &mut rng, &start);
    }
}

fn _recurse<G: MazeGrid>(
    grid: &mut G,
    unvisited: &mut HashSet<<G::CellType as MazeCell>::PositionType>,
    rng: &mut rand::ThreadRng,
    current: &<G::CellType as MazeCell>::PositionType,
) {
    unvisited.remove(current);
    loop {
        let neighbors = grid.neighbors(current);
        let unvisited_neighbors: Vec<<G::CellType as MazeCell>::PositionType> = neighbors
            .iter()
            .filter(|x| unvisited.contains(x))
            .cloned()
            .collect();

        if !unvisited_neighbors.is_empty() {
            if let Some(neighbor) = rng.choose(&unvisited_neighbors) {
                grid.link(current, neighbor);
                _recurse(grid, unvisited, rng, &neighbor.clone());
            }
        } else {
            break;
        }
    }
}

pub fn iterative_backtracker<G: MazeGrid>(grid: &mut G) {
    let mut unvisited: HashSet<<G::CellType as MazeCell>::PositionType> =
        grid.cells().iter().map(|c| c.pos().clone()).collect();
    let mut rng = rand::thread_rng();

    if let Some(start) = _make_initial(&mut unvisited) {
        let mut stack = Vec::new();
        stack.push(start.clone());

        while let Some(cur) = stack.pop() {
            unvisited.remove(&cur);
            let neighbors = grid.neighbors(&cur);
            let unvisited_neighbors: Vec<<G::CellType as MazeCell>::PositionType> = neighbors
                .iter()
                .filter(|x| unvisited.contains(x))
                .cloned()
                .collect();

            if !unvisited_neighbors.is_empty() {
                if let Some(neighbor) = rng.choose(&unvisited_neighbors) {
                    grid.link(&cur, neighbor);
                    stack.push(cur);
                    stack.push(neighbor.clone());
                }
            }
        }
    }
}

pub fn deadends<G: MazeGrid>(grid: &G) -> Vec<<G::CellType as MazeCell>::PositionType> {
    grid.cells()
        .iter()
        .filter(|c| c.links().len() < 2)
        .map(|c| c.pos().clone())
        .collect()
}

pub fn braid<G: MazeGrid>(grid: &mut G, dead_end_removal_probability: f32) {
    let mut rng = rand::thread_rng();

    for pos in deadends(grid) {
        if rng.gen::<f32>() >= dead_end_removal_probability {
            continue;
        }

        let mut choice = None;

        if let Some(cell) = grid.get(&pos) {
            let mut neighbors = cell.neighbors()
                .iter()
                .filter(|p| !cell.links().contains(p))
                .cloned()
                .collect::<Vec<<G::CellType as MazeCell>::PositionType>>();

            let best = neighbors
                .iter()
                .filter(|p| grid.num_links(p) == 1)
                .cloned()
                .collect::<Vec<<G::CellType as MazeCell>::PositionType>>();

            if !best.is_empty() {
                neighbors = best;
            }

            if let Some(ch) = rng.choose(&neighbors) {
                choice = Some(ch.clone());
            }
        }

        if let Some(choice) = choice {
            grid.link(&pos, &choice);
        }
    }
}

pub fn simplified_prims<G: MazeGrid>(grid: &mut G) {
    let mut rng = rand::thread_rng();

    if let Some(start) = grid.random_pos() {
        let mut active = HashSet::new();
        active.insert(start);

        while !active.is_empty() {
            if let Some(pos) = rng.choose(&active
                .iter()
                .cloned()
                .collect::<Vec<<G::CellType as MazeCell>::PositionType>>())
            {
                if let Some(neighbor) = rng.choose(&grid.neighbors(&pos)
                    .iter()
                    .filter(|n| !grid.has_links(n))
                    .collect::<Vec<&<G::CellType as MazeCell>::PositionType>>())
                {
                    grid.link(pos, neighbor);
                    active.insert((*neighbor).clone());
                } else {
                    active.remove(pos);
                }
            }
        }
    }
}

pub fn true_prims<G: MazeGrid>(grid: &mut G) {
    let mut rng = rand::thread_rng();

    if let Some(start) = grid.random_pos() {
        let costs: HashMap<<G::CellType as MazeCell>::PositionType, u8> = HashMap::from_iter(
            grid.cells()
                .iter()
                .map(|c| (c.pos().clone(), rng.gen_range(0, 100))),
        );
        let mut active = HashSet::new();
        active.insert(start);

        while !active.is_empty() {
            if let Some(pos) = active
                .iter()
                .cloned()
                .min_by(|x, y| costs[x].cmp(&costs[y]))
            {
                if let Some(neighbor) = grid.neighbors(&pos)
                    .iter()
                    .filter(|p| !grid.has_links(p))
                    .min_by(|x, y| costs[x].cmp(&costs[y]))
                {
                    grid.link(&pos, neighbor);
                    active.insert((*neighbor).clone());
                } else {
                    active.remove(&pos);
                }
            }
        }
    }
}

pub fn growing_tree<G: MazeGrid, F>(grid: &mut G, selection_fn: F)
where
    F: Fn(&mut LinkedHashSet<<G::CellType as MazeCell>::PositionType>, &mut ThreadRng)
        -> Option<<G::CellType as MazeCell>::PositionType>,
{
    let mut rng = rand::thread_rng();

    if let Some(start) = grid.random_pos() {
        let mut active = LinkedHashSet::new();
        active.insert(start);

        while !active.is_empty() {
            // unwrap since we sort of want to fail hard if we ever don't get
            // something back
            let pos = selection_fn(&mut active, &mut rng).unwrap();

            if let Some(neighbor) = rng.choose(&grid.neighbors(&pos)
                .iter()
                .filter(|n| !grid.has_links(n))
                .collect::<Vec<&<G::CellType as MazeCell>::PositionType>>())
            {
                grid.link(&pos, neighbor);
                active.insert((*neighbor).clone());
            } else {
                active.remove(&pos);
            }
        }
    }
}

pub fn random_selection<G: MazeGrid>(
    active: &mut LinkedHashSet<<G::CellType as MazeCell>::PositionType>,
    rng: &mut ThreadRng,
) -> Option<<G::CellType as MazeCell>::PositionType> {
    match rng.choose(&active
        .iter()
        .collect::<Vec<&<G::CellType as MazeCell>::PositionType>>())
    {
        Some(pos) => Some((*pos).clone()),
        None => None,
    }
}

pub fn last_selection<G: MazeGrid>(
    active: &mut LinkedHashSet<<G::CellType as MazeCell>::PositionType>,
    _: &mut ThreadRng,
) -> Option<<G::CellType as MazeCell>::PositionType> {
    active.back().cloned()
}

pub fn mixed_selection<G: MazeGrid>(
    active: &mut LinkedHashSet<<G::CellType as MazeCell>::PositionType>,
    rng: &mut ThreadRng,
) -> Option<<G::CellType as MazeCell>::PositionType> {
    match rng.gen_range(0, 2) {
        0 => last_selection::<G>(active, rng),
        _ => random_selection::<G>(active, rng),
    }
}
