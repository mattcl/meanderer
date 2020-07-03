#![recursion_limit = "1024"] // error chain
extern crate image;
extern crate imageproc;
extern crate itertools;
extern crate linked_hash_set;
extern crate rand;
extern crate termion;
extern crate tui;

pub mod algorithms;
pub mod data;
pub mod rendering;
pub mod solver;

#[cfg(test)]
mod tests {
    use super::*;
}
