use std::hash::Hash;

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
