use std::collections::HashMap;
use std::fs;
use std::iter::zip;

pub fn day01(input_path: String) {
    let input: String = fs::read_to_string(input_path).unwrap();
    let (mut left, mut right) = parse_input(&input);

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
    let mut counter = HashMap::new();

    for element in right {
        *counter.entry(*element).or_default() += 1;
    }

    left.iter()
        .map(|v| counter.get(v).map(|c| c * v).unwrap_or(0))
        .sum()
}
