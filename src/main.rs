#![feature(cmp_minmax)]

use clap::Parser;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    day: u8,

    #[arg(short, long, default_value = "input")]
    input_path: String,
}

fn main() {
    let args = Args::parse();
    match args.day {
        1 => day01::day01(args.input_path),
        2 => day02::day02(args.input_path),
        3 => day03::day03(args.input_path),
        4 => day04::day04(args.input_path),
        5 => day05::day05(args.input_path),
        6 => day06::day06(args.input_path),
        7 => day07::day07(args.input_path),
        8 => day08::day08(args.input_path),
        9 => day09::day09(args.input_path),
        10 => day10::day10(args.input_path),
        _ => day11::day11(args.input_path),
    }
}
