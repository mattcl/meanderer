#![recursion_limit = "1024"] // error chain
#[macro_use]
extern crate error_chain;
extern crate itertools;
extern crate rand;
extern crate termion;
extern crate tui;

pub mod algorithms;
pub mod data;

#[cfg(test)]
mod tests {
    use super::*;
}
