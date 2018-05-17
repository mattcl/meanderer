use data::pos::Position;
use data::cell::{MazeCell, Cell};
use itertools::Itertools;
use std::iter;

pub trait MazeGrid {
    type CellType: MazeCell;

    fn get(&self, pos: &<Self::CellType as MazeCell>::PositionType) -> Option<&Self::CellType>;
    fn get_mut(
        &mut self,
        pos: &<Self::CellType as MazeCell>::PositionType,
    ) -> Option<&mut Self::CellType>;
    fn contains(&self, pos: &<Self::CellType as MazeCell>::PositionType) -> bool;
    fn neighbors(
        &self,
        pos: &<Self::CellType as MazeCell>::PositionType,
    ) -> Vec<<Self::CellType as MazeCell>::PositionType>;
    fn get_pos(
        &self,
        pos: &<Self::CellType as MazeCell>::PositionType,
    ) -> Option<<Self::CellType as MazeCell>::PositionType>;
    fn link(
        &mut self,
        pos: &<Self::CellType as MazeCell>::PositionType,
        other: &<Self::CellType as MazeCell>::PositionType,
    );
    fn unlink(
        &mut self,
        pos: &<Self::CellType as MazeCell>::PositionType,
        other: &<Self::CellType as MazeCell>::PositionType,
    );
    fn has_links(&self, pos: &<Self::CellType as MazeCell>::PositionType) -> bool;
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

    fn neighbors(&self, pos: &Position) -> Vec<Position> {
        let idx = pos.col + pos.row * self.width;
        match self.cells.get(idx) {
            Some(ref cell) => cell.neighbors(),
            None => Vec::new(),
        }
    }

    fn get_pos(&self, pos: &Position) -> Option<Position> {
        if !self.contains(pos) {
            return None;
        }
        let idx = pos.col + pos.row * self.width;
        self.cells.get(idx).and_then(|cell| Some(cell.pos.clone()))
    }

    fn link(&mut self, pos: &Position, other: &Position) {
        {
            let ref mut root = self.get_mut(pos).unwrap();
            root.link(other);
        }
        {
            let ref mut root = self.get_mut(other).unwrap();
            root.link(pos);
        }
    }

    fn unlink(&mut self, pos: &Position, other: &Position) {
        {
            let ref mut root = self.get_mut(pos).unwrap();
            root.unlink(other);
        }
        {
            let ref mut root = self.get_mut(other).unwrap();
            root.unlink(pos);
        }
    }

    fn has_links(&self, pos: &Position) -> bool {
        let idx = pos.col + pos.row * self.width;
        match self.cells.get(idx) {
            Some(cell) => !cell.links().is_empty(),
            None => false,
        }
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

#[cfg(test)]
mod test {
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
