use data::cell::{Cell, MazeCell, PolarCell};
use data::pos::Position;
use itertools::Itertools;
use std::f32::consts::PI;
use std::iter;

pub trait MazeGrid {
    type CellType: MazeCell;

    fn cells(&self) -> &Vec<Self::CellType>;

    fn get(&self, pos: &<Self::CellType as MazeCell>::PositionType) -> Option<&Self::CellType>;

    fn get_mut(
        &mut self,
        pos: &<Self::CellType as MazeCell>::PositionType,
    ) -> Option<&mut Self::CellType>;

    fn contains(&self, pos: &<Self::CellType as MazeCell>::PositionType) -> bool;

    fn neighbors(
        &self,
        pos: &<Self::CellType as MazeCell>::PositionType,
    ) -> Vec<<Self::CellType as MazeCell>::PositionType> {
        match self.get(pos) {
            Some(ref cell) => cell.neighbors(),
            None => Vec::new(),
        }
    }

    fn get_pos(
        &self,
        pos: &<Self::CellType as MazeCell>::PositionType,
    ) -> Option<<Self::CellType as MazeCell>::PositionType> {
        self.get(pos).and_then(|cell| Some(cell.pos().clone()))
    }

    fn link(
        &mut self,
        pos: &<Self::CellType as MazeCell>::PositionType,
        other: &<Self::CellType as MazeCell>::PositionType,
    ) {
        {
            let ref mut root = self.get_mut(pos).unwrap();
            root.link(other);
        }
        {
            let ref mut root = self.get_mut(other).unwrap();
            root.link(pos);
        }
    }

    fn unlink(
        &mut self,
        pos: &<Self::CellType as MazeCell>::PositionType,
        other: &<Self::CellType as MazeCell>::PositionType,
    ) {
        {
            let ref mut root = self.get_mut(pos).unwrap();
            root.unlink(other);
        }
        {
            let ref mut root = self.get_mut(other).unwrap();
            root.unlink(pos);
        }
    }

    fn has_links(&self, pos: &<Self::CellType as MazeCell>::PositionType) -> bool {
        match self.get(pos) {
            Some(cell) => !cell.links().is_empty(),
            None => false,
        }
    }

    fn to_string(&self, display_labels: bool) -> String;
}

#[derive(Debug, Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = Grid {
            width: width,
            height: height,
            cells: Vec::with_capacity(width * height),
        };

        for row in 0..height {
            for col in 0..width {
                let mut new = Cell::new(row, col);
                if row < height - 1 {
                    new.south = Some(Position::new(row + 1, col))
                }

                if row > 0 {
                    new.north = Some(Position::new(row - 1, col))
                }

                if col > 0 {
                    new.west = Some(Position::new(row, col - 1))
                }

                if col < width - 1 {
                    new.east = Some(Position::new(row, col + 1))
                }

                grid.cells.push(new);
            }
        }

        grid
    }
}

impl MazeGrid for Grid {
    type CellType = Cell;

    fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    fn get(&self, pos: &Position) -> Option<&Cell> {
        if !self.contains(pos) {
            return None;
        }
        let idx = pos.col + pos.row * self.width;
        self.cells.get(idx)
    }

    fn get_mut(&mut self, pos: &Position) -> Option<&mut Cell> {
        if !self.contains(pos) {
            return None;
        }
        let idx = pos.col + pos.row * self.width;
        self.cells.get_mut(idx)
    }

    fn contains(&self, pos: &Position) -> bool {
        // we don't have to check for negative numbers, since usize
        pos.row < self.height && pos.col < self.width
    }

    fn to_string(&self, display_labels: bool) -> String {
        let mut output = String::new();
        output += &iter::repeat("+").take(self.width + 1).join("---");
        output += "\n";

        for row in 0..self.height {
            let mut top = "|".to_string();
            let mut bot = "+".to_string();

            for col in 0..self.width {
                let cur = Position::new(row, col);
                let ref cell = self.get(&cur).unwrap();
                if display_labels {
                    top += &format!("{:^3}", cell.label());
                } else {
                    top += "   ";
                }

                if col < self.width - 1 {
                    let east = self.get(&Position::new(cur.row, cur.col + 1)).unwrap();
                    if cell.is_linked(east) {
                        top += " ";
                    } else {
                        top += "|";
                    }
                } else {
                    top += "|";
                }

                if row < self.height - 1 {
                    let south = self.get(&Position::new(cur.row + 1, cur.col)).unwrap();
                    if cell.is_linked(south) {
                        bot += "   +";
                    } else {
                        bot += "---+";
                    }
                } else {
                    bot += "---+";
                }
            }

            top += "\n";
            bot += "\n";

            output += &top;
            output += &bot;
        }
        output
    }
}

#[derive(Debug, Clone)]
pub struct PolarGrid {
    // like with the row-major stuff, i want to store the number of columns
    // per all rows before me so i can efficiently calculate the offset given a row
    //     |     |              |
    // [a, b, c, d, e, f, g, h, i, j]
    //
    // [0, 1,    3              8]
    //
    // i also need to store the number of columns per row
    // [1, 2,    5,             2]
    pub rows: usize,
    pub cells: Vec<PolarCell>,
    pub row_offsets: Vec<usize>,
    pub column_counts: Vec<usize>,
}

impl PolarGrid {
    pub fn new(rows: usize) -> Self {
        let mut grid = PolarGrid {
            rows: rows,
            cells: Vec::new(),
            row_offsets: Vec::with_capacity(rows),
            column_counts: Vec::with_capacity(rows),
        };

        grid._make_cells();
        grid._set_neighbors();

        grid
    }

    fn _make_cells(&mut self) {
        self.cells.push(PolarCell::new(0, 0));
        self.row_offsets.push(0);
        self.column_counts.push(1);

        let row_height = 1.0 / self.rows as f32;

        for row in 1..self.rows {
            let radius = row as f32 / self.rows as f32;
            let circumference = 2.0 * PI * radius;

            let prev_cols = self.column_counts[row - 1];
            let est_cell_width = circumference / prev_cols as f32;
            let ratio = (est_cell_width / row_height).round() as usize;
            let num_cols = ratio * self.column_counts[row - 1];

            let previous_offset = self.row_offsets[row - 1];
            self.row_offsets.push(previous_offset + prev_cols);
            self.column_counts.push(num_cols);

            for col in 0..num_cols {
                self.cells.push(PolarCell::new(row, col));
            }
        }
    }

    fn _set_neighbors(&mut self) {
        for i in 0..self.cells.len() {
            let pos = self.cells[i].pos().clone();
            if pos.row > 0 {
                let num_cols = self.column_counts[pos.row];
                let ratio = num_cols / self.column_counts[pos.row - 1];
                let parent_pos = Position::new(pos.row - 1, pos.col / ratio);

                if let Some(ref mut cell) = self.get_mut(&pos) {
                    let cw_col = if pos.col == num_cols - 1 {
                        0
                    } else {
                        pos.col + 1
                    };

                    let ccw_col = if pos.col == 0 {
                        num_cols - 1
                    } else {
                        pos.col - 1
                    };

                    cell.cw = Some(Position::new(pos.row, cw_col));
                    cell.ccw = Some(Position::new(pos.row, ccw_col));

                    cell.inward = Some(parent_pos.clone());
                }

                if let Some(ref mut cell) = self.get_mut(&parent_pos) {
                    cell.outward.push(pos);
                }
            }
        }
    }
}

impl MazeGrid for PolarGrid {
    type CellType = PolarCell;

    fn cells(&self) -> &Vec<PolarCell> {
        &self.cells
    }

    fn get(&self, pos: &Position) -> Option<&PolarCell> {
        if !self.contains(pos) {
            return None;
        }
        let idx = pos.col + self.row_offsets[pos.row];
        self.cells.get(idx)
    }

    fn get_mut(&mut self, pos: &Position) -> Option<&mut PolarCell> {
        if !self.contains(pos) {
            return None;
        }
        let idx = pos.col + self.row_offsets[pos.row];
        self.cells.get_mut(idx)
    }

    fn contains(&self, pos: &Position) -> bool {
        // we don't have to check for negative numbers, since usize
        pos.row < self.rows && pos.col < self.column_counts[pos.row]
    }

    fn to_string(&self, _: bool) -> String {
        "to_string is meaningless for polar grids".to_owned()
    }
}

#[cfg(test)]
mod test_grid {
    use super::*;

    #[test]
    fn new() {
        let width = 2;
        let height = 3;
        let a = Grid::new(width, height);

        assert_eq!(a.width, width);
        assert_eq!(a.height, height);

        for row in 0..height {
            for col in 0..width {
                let cell = a.get(&Position::new(row, col)).unwrap();
                if row < height - 1 {
                    assert_eq!(cell.south, Some(Position::new(row + 1, col)));
                } else {
                    assert_eq!(cell.south, None);
                }

                if row > 0 {
                    assert_eq!(cell.north, Some(Position::new(row - 1, col)));
                } else {
                    assert_eq!(cell.north, None);
                }

                if col > 0 {
                    assert_eq!(cell.west, Some(Position::new(row, col - 1)));
                } else {
                    assert_eq!(cell.west, None);
                }

                if col < width - 1 {
                    assert_eq!(cell.east, Some(Position::new(row, col + 1)));
                } else {
                    assert_eq!(cell.east, None);
                }

                assert_eq!(*cell, Cell::new(row, col));
            }
        }
    }

    #[test]
    fn neighbors() {
        let width = 2;
        let height = 3;
        let grid = Grid::new(width, height);

        let a = grid.get(&Position::new(1, 1)).unwrap();
        assert_eq!(grid.neighbors(&a.pos), a.neighbors());
    }

    #[test]
    fn contains() {
        let width = 2;
        let height = 3;
        let grid = Grid::new(width, height);

        assert!(!grid.contains(&Position::new(height, width)));
        assert!(!grid.contains(&Position::new(height - 1, width)));
        assert!(!grid.contains(&Position::new(height, width - 1)));
        assert!(grid.contains(&Position::new(height - 1, width - 1)));
        assert!(grid.contains(&Position::new(0, 0)));
    }

    #[test]
    fn getting() {
        let width = 2;
        let height = 3;
        let grid = Grid::new(width, height);

        {
            let a = grid.get(&Position::new(2, 1));
            assert!(a.is_some());
            assert_eq!(a.unwrap(), &Cell::new(2, 1));

            let a = grid.get(&Position::new(0, 0));
            assert!(a.is_some());
            assert_eq!(a.unwrap(), &Cell::new(0, 0));

            let a = grid.get(&Position::new(3, 1));
            assert!(a.is_none());

            let a = grid.get(&Position::new(0, 2));
            assert!(a.is_none());
        }
    }

    #[test]
    fn getting_as_mut() {
        let width = 2;
        let height = 3;
        let mut grid = Grid::new(width, height);

        {
            let a = grid.get_mut(&Position::new(2, 1));
            assert!(a.is_some());
            assert_eq!(a.unwrap(), &Cell::new(2, 1));
        }

        // verify we can change things
        {
            {
                let a = grid.get_mut(&Position::new(2, 1)).unwrap();
                a.update_weight(10);
            }
            {
                let b = grid.get(&Position::new(2, 1)).unwrap();
                assert_eq!(b.weight(), 10);
            }
        }

        {
            let a = grid.get_mut(&Position::new(0, 0));
            assert!(a.is_some());
            assert_eq!(a.unwrap(), &Cell::new(0, 0));
        }

        {
            let a = grid.get_mut(&Position::new(3, 1));
            assert!(a.is_none());
        }

        {
            let a = grid.get_mut(&Position::new(0, 2));
            assert!(a.is_none());
        }
    }

    #[test]
    fn getting_positions() {
        let width = 2;
        let height = 3;
        let grid = Grid::new(width, height);

        {
            let a = grid.get_pos(&Position::new(2, 1));
            assert!(a.is_some());
            assert_eq!(a.unwrap(), Position::new(2, 1));

            let a = grid.get_pos(&Position::new(0, 0));
            assert!(a.is_some());
            assert_eq!(a.unwrap(), Position::new(0, 0));

            let a = grid.get_pos(&Position::new(3, 1));
            assert!(a.is_none());

            let a = grid.get_pos(&Position::new(0, 2));
            assert!(a.is_none());
        }
    }

    #[test]
    fn linking() {
        let width = 2;
        let height = 3;
        let mut grid = Grid::new(width, height);

        let a = grid.get_pos(&Position::new(0, 0)).unwrap();
        let b = grid.get_pos(&Position::new(1, 0)).unwrap();

        grid.link(&a, &b);

        let a = grid.get(&Position::new(0, 0)).unwrap();
        let b = grid.get(&Position::new(1, 0)).unwrap();
        let c = grid.get(&Position::new(1, 1)).unwrap();

        assert!(a.is_linked(b));
        assert!(b.is_linked(a));
        assert!(!a.is_linked(c));
        assert!(!b.is_linked(c));
    }

    #[test]
    fn unlinking() {
        let width = 2;
        let height = 3;
        let mut grid = Grid::new(width, height);

        let a = grid.get_pos(&Position::new(0, 0)).unwrap();
        let b = grid.get_pos(&Position::new(1, 0)).unwrap();

        grid.link(&a, &b);
        grid.unlink(&a, &b);

        let a = grid.get(&Position::new(0, 0)).unwrap();
        let b = grid.get(&Position::new(1, 0)).unwrap();

        assert!(!a.is_linked(b));
        assert!(!b.is_linked(a));
    }

    #[test]
    fn to_string_base() {
        let width = 2;
        let height = 3;
        let mut grid = Grid::new(width, height);
        let expected = "\
+---+---+
| 0 | 0 |
+---+---+
| 0 | 0 |
+---+---+
| 0 | 0 |
+---+---+
"
            .to_string();

        assert_eq!(grid.to_string(true), expected);

        let expected = "\
+---+---+
|   |   |
+---+---+
|   |   |
+---+---+
|   |   |
+---+---+
"
            .to_string();

        assert_eq!(grid.to_string(false), expected);

        {
            let ref mut p = grid.get_mut(&Position::new(1, 1)).unwrap();
            p.update_weight(2);
        }

        {
            let ref mut p = grid.get_mut(&Position::new(0, 1)).unwrap();
            p.update_weight(13);
        }

        {
            let ref mut p = grid.get_mut(&Position::new(2, 0)).unwrap();
            p.update_weight(456);
        }

        let a = grid.get_pos(&Position::new(0, 0)).unwrap();
        let b = grid.get_pos(&Position::new(0, 1)).unwrap();
        let c = grid.get_pos(&Position::new(1, 0)).unwrap();
        let d = grid.get_pos(&Position::new(2, 0)).unwrap();
        let e = grid.get_pos(&Position::new(2, 1)).unwrap();

        grid.link(&a, &b);
        grid.link(&c, &d);
        grid.link(&d, &e);

        let expected = "\
+---+---+
| 0  13 |
+---+---+
| 0 | 2 |
+   +---+
|456  0 |
+---+---+
"
            .to_string();

        assert_eq!(grid.to_string(true), expected);

        let expected = "\
+---+---+
|       |
+---+---+
|   |   |
+   +---+
|       |
+---+---+
"
            .to_string();

        assert_eq!(grid.to_string(false), expected);
    }

}

#[cfg(test)]
mod test_polar_grid {
    use super::*;

    #[test]
    fn new() {
        let rows = 4;
        let a = PolarGrid::new(rows);

        assert_eq!(a.rows, rows);

        assert_eq!(a.neighbors(&Position::new(2, 1)).len(), 5);
    }

    #[test]
    fn neighbors() {
        let rows = 4;
        let grid = PolarGrid::new(rows);

        let a = grid.get(&Position::new(1, 1)).unwrap();
        assert_eq!(grid.neighbors(&a.pos), a.neighbors());
    }

    #[test]
    fn contains() {
        let rows = 4;
        let grid = PolarGrid::new(rows);

        assert!(grid.contains(&Position::new(0, 0)));
        assert!(!grid.contains(&Position::new(0, 1)));
        assert!(!grid.contains(&Position::new(1, 20)));
        assert!(grid.contains(&Position::new(1, 0)));
        assert!(grid.contains(&Position::new(2, 2)));
    }

    #[test]
    fn getting() {
        let rows = 4;
        let grid = PolarGrid::new(rows);

        {
            let a = grid.get(&Position::new(2, 1));
            assert!(a.is_some());
            assert_eq!(a.unwrap(), &PolarCell::new(2, 1));

            let a = grid.get(&Position::new(0, 0));
            assert!(a.is_some());
            assert_eq!(a.unwrap(), &PolarCell::new(0, 0));

            let a = grid.get(&Position::new(0, 2));
            assert!(a.is_none());
        }
    }

    #[test]
    fn getting_as_mut() {
        let rows = 4;
        let mut grid = PolarGrid::new(rows);

        {
            let a = grid.get_mut(&Position::new(2, 1));
            assert!(a.is_some());
            assert_eq!(a.unwrap(), &PolarCell::new(2, 1));
        }

        // verify we can change things
        {
            {
                let a = grid.get_mut(&Position::new(2, 1)).unwrap();
                a.update_weight(10);
            }
            {
                let b = grid.get(&Position::new(2, 1)).unwrap();
                assert_eq!(b.weight(), 10);
            }
        }

        {
            let a = grid.get_mut(&Position::new(0, 0));
            assert!(a.is_some());
            assert_eq!(a.unwrap(), &PolarCell::new(0, 0));
        }

        {
            let a = grid.get_mut(&Position::new(0, 2));
            assert!(a.is_none());
        }
    }

    #[test]
    fn getting_positions() {
        let rows = 4;
        let grid = PolarGrid::new(rows);

        {
            let a = grid.get_pos(&Position::new(2, 1));
            assert!(a.is_some());
            assert_eq!(a.unwrap(), Position::new(2, 1));

            let a = grid.get_pos(&Position::new(0, 0));
            assert!(a.is_some());
            assert_eq!(a.unwrap(), Position::new(0, 0));

            let a = grid.get_pos(&Position::new(0, 2));
            assert!(a.is_none());
        }
    }

    #[test]
    fn linking() {
        let rows = 4;
        let mut grid = PolarGrid::new(rows);

        let a = grid.get_pos(&Position::new(0, 0)).unwrap();
        let b = grid.get_pos(&Position::new(1, 0)).unwrap();

        grid.link(&a, &b);

        let a = grid.get(&Position::new(0, 0)).unwrap();
        let b = grid.get(&Position::new(1, 0)).unwrap();
        let c = grid.get(&Position::new(1, 1)).unwrap();

        assert!(a.is_linked(b));
        assert!(b.is_linked(a));
        assert!(!a.is_linked(c));
        assert!(!b.is_linked(c));
    }

    #[test]
    fn unlinking() {
        let rows = 4;
        let mut grid = PolarGrid::new(rows);

        let a = grid.get_pos(&Position::new(0, 0)).unwrap();
        let b = grid.get_pos(&Position::new(1, 0)).unwrap();

        grid.link(&a, &b);
        grid.unlink(&a, &b);

        let a = grid.get(&Position::new(0, 0)).unwrap();
        let b = grid.get(&Position::new(1, 0)).unwrap();

        assert!(!a.is_linked(b));
        assert!(!b.is_linked(a));
    }

    #[test]
    fn to_string_base() {
        let rows = 4;
        let a = PolarGrid::new(rows);

        assert_eq!(
            a.to_string(false),
            "to_string is meaningless for polar grids".to_owned()
        );
    }

}
