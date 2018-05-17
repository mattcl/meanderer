use data::{Grid, MazeGrid, Position};
use std::collections::HashSet;

pub fn dijkstra(grid: &mut Grid, start: &Position) {
    let mut visited = HashSet::new();
    let mut front: Vec<Position> = vec![start.clone()];
    let mut dist = 0;

    while !front.is_empty() {
        let mut next = Vec::new();
        for pos in &front {
            visited.insert(pos.clone());
            if let Some(ref mut cell) = grid.get_mut(pos) {
                cell.weight = dist;
                let mut links = cell.links
                    .iter()
                    .cloned()
                    .filter(|l| !visited.contains(l))
                    .collect::<Vec<Position>>();

                next.append(&mut links);
            }
        }
        front = next;
        dist += 1;
    }
}

pub fn dijkstra_recursive(grid: &mut Grid, start: &Position) {
    let start = vec![start.clone()];
    let mut visited = HashSet::new();
    _dijkstra(grid, start, 0, &mut visited);
}

fn _dijkstra(grid: &mut Grid, front: Vec<Position>, dist: u32, visited: &mut HashSet<Position>) {
    if front.is_empty() {
        return;
    }

    let mut next = Vec::new();

    for pos in front {
        visited.insert(pos.clone());
        if let Some(ref mut cell) = grid.get_mut(&pos) {
            cell.weight = dist;
            let mut links = cell.links
                .iter()
                .cloned()
                .filter(|l| !visited.contains(l))
                .collect::<Vec<Position>>();

            next.append(&mut links);
        }
    }

    _dijkstra(grid, next, dist + 1, visited);
}

pub fn furthest_corners(grid: &mut Grid) -> (Position, Position) {
    let mut candidates = Vec::new();

    let corners = vec![
        grid.get(&Position::new(0, 0)).unwrap().clone(),
        grid.get(&Position::new(0, grid.width - 1)).unwrap().clone(),
        grid.get(&Position::new(grid.height - 1, 0)).unwrap().clone(),
        grid.get(&Position::new(grid.height - 1, grid.width - 1)).unwrap().clone(),
    ];

    for corner in &corners {
        dijkstra(grid, &corner.pos);
        let max  = vec![
            grid.get(&Position::new(0, 0)).unwrap(),
            grid.get(&Position::new(0, grid.width - 1)).unwrap(),
            grid.get(&Position::new(grid.height - 1, 0)).unwrap(),
            grid.get(&Position::new(grid.height - 1, grid.width - 1)).unwrap(),
        ].iter().max_by_key(|c| c.weight).unwrap().clone();
        candidates.push((corner.pos.clone(), max.pos.clone(), max.weight));
    }

    candidates.sort_by_key(|c| c.2);

    let (pos1, pos2, _) = candidates[candidates.len() - 1].clone();
    (pos1, pos2)
}

pub fn solve(grid: &mut Grid, start: &Position, target: &Position) {
    dijkstra(grid, start);
    _solve_to(grid, target);
}

fn _solve_to(grid: &mut Grid, target: &Position) {
    let mut next = Some(target.clone());

    while let Some(cur) = next.clone() {
        next = None;
        if let Some(ref mut cur) = grid.get_mut(&cur) {
            cur.in_solution = true;
        }

        if let Some(ref cur) = grid.get(&cur) {
            for link in &cur.links {
                if let Some(ref cell) = grid.get(&link) {
                    if cell.weight < cur.weight {
                        next = Some(link.clone());
                        break;
                    }
                }
            }
        }
    }
}
