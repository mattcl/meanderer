use data::{Grid, Position};
use std::collections::HashSet;


pub fn dijkstra(grid: &mut Grid, start: &Position) {
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
        if let Some(ref mut cell) = grid.get_mut(pos.row, pos.col) {
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

pub fn solve(grid: &mut Grid, start: &Position, target: &Position) {
    dijkstra(grid, start);
    _solve_to(grid, target);
}

fn _solve_to(grid: &mut Grid, target: &Position) {
    let mut next = None;

    if let Some(ref mut target) = grid.get_mut(target.row, target.col) {
        target.in_solution = true;
    }

    if let Some(ref target) = grid.get(target.row, target.col) {
        for link in &target.links {
            if let Some(ref cell) = grid.get(link.row, link.col) {
                if cell.weight < target.weight {
                    next = Some(link.clone());
                }
            }
        }
    }

    if let Some(next) = next {
        _solve_to(grid, &next);
    }
}
