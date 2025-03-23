#![feature(cmp_minmax)]

use clap::Parser;
use std::path::PathBuf;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    day: u8,
}

fn main() {
    let args = Args::parse();
    let input = PathBuf::from("input");
    match args.day {
        1 => day01::day01(input),
        2 => day02::day02(input),
        3 => day03::day03(input),
        4 => day04::day04(input),
        5 => day05::day05(input),
        6 => day06::day06(input),
        7 => day07::day07(input),
        _ => day08::day08(input),
    }
}
