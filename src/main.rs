#![feature(btree_cursors)]

use std::path::PathBuf;

// mod day01;
// mod day02;
// mod day03;
mod day06;

fn main() {
    let input = PathBuf::from("input");
    day06::day06(input);
}
