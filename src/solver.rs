use data::cell::MazeCell;
use data::grid::{Grid, MazeGrid, PolarGrid};
use data::pos::Position;
use std::collections::HashSet;

pub fn dijkstra<G: MazeGrid>(grid: &mut G, start: &<G::CellType as MazeCell>::PositionType) {
    let mut visited = HashSet::new();
    let mut front: Vec<<G::CellType as MazeCell>::PositionType> = vec![start.clone()];
    let mut dist = 0;

    while !front.is_empty() {
        let mut next = Vec::new();
        for pos in &front {
            // possible with loops
            if visited.contains(pos) {
                continue;
            }

            visited.insert(pos.clone());
            if let Some(ref mut cell) = grid.get_mut(pos) {
                cell.update_weight(dist);
                let mut links = cell.links()
                    .iter()
                    .cloned()
                    .filter(|l| !visited.contains(l))
                    .collect::<Vec<<G::CellType as MazeCell>::PositionType>>();

                next.append(&mut links);
            }
        }
        front = next;
        dist += 1;
    }
}

pub fn solve<G: MazeGrid>(
    grid: &mut G,
    start: &<G::CellType as MazeCell>::PositionType,
    target: &<G::CellType as MazeCell>::PositionType,
) {
    dijkstra(grid, start);

    let mut next = Some(target.clone());

    while let Some(cur) = next.clone() {
        next = None;
        if let Some(ref mut cur) = grid.get_mut(&cur) {
            cur.mark_in_solution();
        }

        if let Some(ref cur) = grid.get(&cur) {
            for link in cur.links() {
                if let Some(ref cell) = grid.get(&link) {
                    if cell.weight() < cur.weight() {
                        next = Some(link.clone());
                        break;
                    }
                }
            }
        }
    }
}

pub fn furthest_on_rim(grid: &mut PolarGrid, from: &Position) -> Position {
    dijkstra(grid, from);
    let max_row = grid.rows - 1;
    let num_cols = grid.column_counts[max_row];

    let mut max = 0;
    let mut max_pos = Position::new(max_row, 0);
    for col in 0..num_cols {
        let pos = Position::new(max_row, col);
        let cell = grid.get(&pos).unwrap();
        if cell.weight() > max {
            max = cell.weight();
            max_pos = pos;
        }
    }

    grid.get(&max_pos).unwrap().pos().clone()
}

pub fn furthest_corners(grid: &mut Grid) -> (Position, Position) {
    let mut candidates = Vec::new();

    let corners = vec![
        grid.get(&Position::new(0, 0)).unwrap().clone(),
        grid.get(&Position::new(0, grid.width - 1)).unwrap().clone(),
        grid.get(&Position::new(grid.height - 1, 0))
            .unwrap()
            .clone(),
        grid.get(&Position::new(grid.height - 1, grid.width - 1))
            .unwrap()
            .clone(),
    ];

    for corner in &corners {
        dijkstra(grid, &corner.pos);
        let max = vec![
            grid.get(&Position::new(0, 0)).unwrap(),
            grid.get(&Position::new(0, grid.width - 1)).unwrap(),
            grid.get(&Position::new(grid.height - 1, 0)).unwrap(),
            grid.get(&Position::new(grid.height - 1, grid.width - 1))
                .unwrap(),
        ].iter()
            .max_by_key(|c| c.weight())
            .unwrap()
            .clone();
        candidates.push((corner.pos.clone(), max.pos.clone(), max.weight()));
    }

    candidates.sort_by_key(|c| c.2);

    let (pos1, pos2, _) = candidates[candidates.len() - 1].clone();
    (pos1, pos2)
}
