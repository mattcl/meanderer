use itertools::Itertools;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};
use std::iter;

pub trait MazePosition: Clone + Eq + Hash + Ord + PartialEq + PartialOrd {}

#[derive(Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Self {
        Position { row: row, col: col }
    }
}

impl MazePosition for Position {}

pub trait MazeCell {
    type PositionType: MazePosition;

    fn pos(&self) -> &Self::PositionType;
    fn label(&self) -> String;
    fn link(&mut self, other: &Self::PositionType);
    fn unlink(&mut self, other: &Self::PositionType);
    fn is_linked(&self, other: &Self) -> bool;
    fn is_linked_pos(&self, other: &Self::PositionType) -> bool;
    fn links(&self) -> &BTreeSet<Self::PositionType>;
    fn neighbors(&self) -> Vec<Self::PositionType>;
    fn weight(&self) -> u32;
    fn update_weight(&mut self, weight: u32);
    fn in_solution(&self) -> bool;
    fn mark_in_solution(&mut self);
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub pos: Position,
    pub north: Option<Position>,
    pub south: Option<Position>,
    pub east: Option<Position>,
    pub west: Option<Position>,

    weight: u32,
    in_solution: bool,
    links: BTreeSet<Position>,
}

impl Cell {
    pub fn new(row: usize, col: usize) -> Cell {
        Cell {
            pos: Position::new(row, col),
            weight: 0,
            in_solution: false,
            north: None,
            south: None,
            east: None,
            west: None,
            links: BTreeSet::new(),
        }
    }
}

impl MazeCell for Cell {
    type PositionType = Position;

    fn pos(&self) -> &Position {
        &self.pos
    }

    fn label(&self) -> String {
        format!("{}", self.weight)
    }

    fn link(&mut self, other: &Position) {
        self.links.insert(other.clone());
    }

    fn unlink(&mut self, other: &Position) {
        self.links.remove(other);
    }

    fn is_linked(&self, other: &Self) -> bool {
        self.links.contains(other.pos())
    }

    fn is_linked_pos(&self, other: &Position) -> bool {
        self.links.contains(other)
    }

    fn links(&self) -> &BTreeSet<Position> {
        &self.links
    }

    fn neighbors(&self) -> Vec<Position> {
        let mut neighbors = Vec::new();

        if let Some(ref pos) = self.north {
            neighbors.push(pos.clone());
        }

        if let Some(ref pos) = self.south {
            neighbors.push(pos.clone());
        }

        if let Some(ref pos) = self.east {
            neighbors.push(pos.clone());
        }

        if let Some(ref pos) = self.west {
            neighbors.push(pos.clone());
        }

        neighbors
    }

    fn weight(&self) -> u32 {
        self.weight
    }

    fn update_weight(&mut self, weight: u32) {
        self.weight = weight;
    }

    fn in_solution(&self) -> bool {
        self.in_solution
    }

    fn mark_in_solution(&mut self) {
        self.in_solution = true;
    }
}

impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.row.hash(state);
        self.pos.col.hash(state);
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.pos == other.pos
    }
}

impl Eq for Cell {}

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
            Some(cell) => !cell.links.is_empty(),
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
mod test_cell {
    use super::*;

    #[test]
    fn new() {
        let a = Cell::new(1, 2);
        let b = Cell {
            pos: Position::new(1, 2),
            weight: 0,
            in_solution: false,
            north: None,
            south: None,
            east: None,
            west: None,
            links: BTreeSet::new(),
        };
        assert_eq!(a, b);
        assert_eq!(a.weight(), b.weight());
    }

    #[test]
    fn equality() {
        let a = Cell {
            pos: Position::new(10, 20),
            weight: 1,
            in_solution: false,
            north: None,
            south: None,
            east: None,
            west: None,
            links: BTreeSet::new(),
        };

        let b = Cell {
            pos: Position::new(10, 20),
            weight: 2,
            in_solution: true,
            north: None,
            south: None,
            east: None,
            west: None,
            links: BTreeSet::new(),
        };

        let c = Cell {
            pos: Position::new(30, 40),
            weight: 1,
            in_solution: true,
            north: None,
            south: None,
            east: None,
            west: None,
            links: BTreeSet::new(),
        };

        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn neighbors() {
        let mut a = Cell::new(3, 4);
        a.north = Some(Position::new(2, 4));

        {
            assert_eq!(a.neighbors(), vec![Position::new(2, 4)]);
        }

        a.south = Some(Position::new(4, 4));

        {
            assert_eq!(
                a.neighbors(),
                vec![Position::new(2, 4), Position::new(4, 4)]
            );
        }

        a.east = Some(Position::new(3, 5));

        {
            assert_eq!(
                a.neighbors(),
                vec![
                    Position::new(2, 4),
                    Position::new(4, 4),
                    Position::new(3, 5),
                ]
            );
        }

        a.west = Some(Position::new(3, 3));

        {
            assert_eq!(
                a.neighbors(),
                vec![
                    Position::new(2, 4),
                    Position::new(4, 4),
                    Position::new(3, 5),
                    Position::new(3, 3),
                ]
            );
        }

        a.south = None;

        {
            assert_eq!(
                a.neighbors(),
                vec![
                    Position::new(2, 4),
                    Position::new(3, 5),
                    Position::new(3, 3),
                ]
            );
        }
    }

    #[test]
    fn linking() {
        let mut a = Cell::new(10, 20);
        let mut b = Cell::new(30, 40);
        let mut c = Cell::new(50, 60);

        a.link(&b.pos);
        b.link(&a.pos);
        b.link(&c.pos);
        c.link(&b.pos);

        assert!(a.is_linked(&b));
        assert!(b.is_linked(&a));
        assert!(b.is_linked(&c));
        assert!(c.is_linked(&b));
        assert!(!a.is_linked(&c));
        assert!(!c.is_linked(&a));

        assert!(a.is_linked_pos(&b.pos));
        assert!(b.is_linked_pos(&a.pos));
        assert!(b.is_linked_pos(&c.pos));
        assert!(c.is_linked_pos(&b.pos));
        assert!(!a.is_linked_pos(&c.pos));
        assert!(!c.is_linked_pos(&a.pos));
    }

    #[test]
    fn unlinking() {
        let mut a = Cell::new(10, 20);
        let b = Cell::new(30, 40);

        a.link(&b.pos);

        a.unlink(&b.pos);

        assert!(!a.is_linked(&b));
        assert!(!a.is_linked_pos(&b.pos));
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
