#![feature(cmp_minmax)]
#![feature(option_zip)]

#[macro_use]
extern crate static_assertions;

use std::{fmt::Debug, fs, time::Instant};

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
mod day12;
mod day13;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    day: u8,

    #[arg(short, long, default_value = "input")]
    input_base: String,

    #[arg(short, long)]
    test: bool,
}

fn timed<F, R>(f: F, path: &str, label: &str)
where
    F: Fn(&str) -> R,
    R: Debug,
{
    let n = 5;
    let input = fs::read_to_string(path).unwrap();
    let start = Instant::now();
    for _ in 1..n {
        f(&input);
    }
    let res = f(&input);
    let end = Instant::now();
    println!("{}: {:?}", label, res);
    println!("Average: {:.2?}", (end - start) / 5);
}

fn call_timed(fn_and_label: (Solution, &str), base: &str, test: bool) {
    timed(
        fn_and_label.0,
        &format!(
            "{}/{}{}.txt",
            base,
            fn_and_label.1,
            if test { "_test" } else { "" }
        ),
        fn_and_label.1,
    );
}

type Solution = fn(&str) -> (usize, usize);

fn main() {
    let args = Args::parse();

    let fn_and_labels: Vec<(Solution, &str)> = vec![
        (day01::day01, "day01"),
        (day02::day02, "day02"),
        (day03::day03, "day03"),
        (day04::day04, "day04"),
        (day05::day05, "day05"),
        (day06::day06, "day06"),
        (day07::day07, "day07"),
        (day08::day08, "day08"),
        (day09::day09, "day09"),
        (day10::day10, "day10"),
        (day11::day11, "day11"),
        (day12::day12, "day12"),
        (day13::day13, "day13"),
    ];

    // underflow is fine
    let (index, _) = args.day.overflowing_sub(1);
    if let Some(&fn_and_label) = fn_and_labels.get(index as usize) {
        call_timed(fn_and_label, &args.input_base, args.test);
    } else {
        println!("Solving all...");
        fn_and_labels
            .iter()
            .for_each(|&fn_and_label| call_timed(fn_and_label, &args.input_base, args.test));
    }
}
