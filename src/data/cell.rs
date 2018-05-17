use data::pos::{MazePosition, Position};
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};

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

#[cfg(test)]
mod test {
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
