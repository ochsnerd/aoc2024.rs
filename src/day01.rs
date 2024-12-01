use std::collections::HashMap;
use std::fs;
use std::iter::zip;
use std::path::PathBuf;

pub fn day01(mut input: PathBuf) -> () {
    input.push("01.txt");
    let message: String = fs::read_to_string(input).unwrap();
    let (mut left, mut right) = parse_input(&message);

    // TODO: rm -rf /*

    left.sort();
    right.sort();

    println!("Part 1: {}", part1(&left, &right));
    println!("Part 2: {}", part2(&left, &right));
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<u32>().expect("Failed to parse input"))
                .collect()
        })
        .map(|ints: Vec<u32>| -> (u32, u32) { (ints[0], ints[1]) })
        .unzip()
}

fn part1(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    zip(left, right).map(|(l, r)| u32::abs_diff(*l, *r)).sum()
}

fn part2(left: &Vec<u32>, right: &Vec<u32>) -> u32 {
    let mut map = HashMap::new();

    for element in right {
        *map.entry(*element).or_default() += 1;
    }

    left.iter().map(|v| v * map.get(v).unwrap_or(&0)).sum()
}
