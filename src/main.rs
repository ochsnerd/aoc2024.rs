
use std::path::PathBuf;
use clap::Parser;

mod day01;
mod day02;
mod day03;
mod day06;

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
	_ => day06::day06(input),
    }
}
