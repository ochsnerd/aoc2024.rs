use std::{
    cmp,
    collections::{HashMap, HashSet},
};

use itertools::iproduct;

pub fn day08(input: &str) -> (usize, usize) {
    let (antennas, corner) = read(&input);

    (
        count_antinodes(&antennas, ((0, 0), corner), antinodes_part1),
        count_antinodes(&antennas, ((0, 0), corner), antinodes_part2),
    )
}

// Positive y-direction is up!
type Position = (i32, i32);
type ResonancePoints = HashMap<char, Vec<Position>>;

fn read(input: &str) -> (ResonancePoints, (i32, i32)) {
    let (mut x_max, mut y_max) = (0, 0);
    let mut antennas = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        y_max = y_max.max(y);
        for (x, c) in line.chars().enumerate() {
            x_max = x_max.max(x);
            if c != '.' {
                antennas
                    .entry(c)
                    .or_insert(Vec::new())
                    .push((x as i32, -(y as i32)))
            }
        }
    }
    (antennas, (x_max as i32, -(y_max as i32)))
}

fn count_antinodes(
    antennas: &ResonancePoints,
    bounds: (Position, Position),
    generate_antinodes: fn(&[Position], (Position, Position)) -> Vec<Position>,
) -> usize {
    antennas
        .iter()
        .flat_map(|(_, positions)| generate_antinodes(&positions, bounds))
        .collect::<HashSet<_>>()
        .len()
}

fn antinodes_part1(antennas: &[Position], bounds: (Position, Position)) -> Vec<Position> {
    iproduct!(antennas, antennas)
        .filter(|(p1, p2)| p1 != p2)
        .map(|((x1, y1), (x2, y2))| (2 * x1 - x2, 2 * y1 - y2))
        .filter(|p| is_inside(*p, bounds))
        .collect()
}

fn antinodes_part2(antennas: &[Position], bounds: (Position, Position)) -> Vec<Position> {
    iproduct!(antennas, antennas)
        .filter(|(p1, p2)| p1 != p2)
        .flat_map(|((x1, y1), (x2, y2))| {
            (0..)
                .map(move |i| ((i + 1) * x1 - i * x2, (i + 1) * y1 - i * y2))
                .take_while(|p| is_inside(*p, bounds))
        })
        .collect()
}

fn is_inside(p: Position, bounds: (Position, Position)) -> bool {
    // ugh
    let (x, y) = p;
    let ((x1, y1), (x2, y2)) = bounds;
    let [x_min, x_max] = cmp::minmax(x1, x2);
    let [y_min, y_max] = cmp::minmax(y1, y2);
    x_min <= x && x <= x_max && y_min <= y && y <= y_max
}
