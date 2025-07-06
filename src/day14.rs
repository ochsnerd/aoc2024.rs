use std::{collections::HashMap, hash::Hash, num::ParseIntError, str::FromStr};

use itertools::Itertools;
use thiserror::Error;

pub fn solve(input: &str) -> (usize, usize) {
    let robots: Vec<_> = input.lines().map(|l| l.parse().unwrap()).collect();
    (part1(&robots), part2(robots).unwrap())
}

type Coord = i16;
type Pos = [Coord; 2];
type Vel = [Coord; 2];

const EXTENT: Pos = [101, 103];

fn part1(robots: &[Robot]) -> usize {
    robots
        .into_iter()
        .map(|r| r.evolved_by(100).pos)
        .filter_map(|p| quadrant(p))
        .counted()
        .into_values()
        .product()
}

fn part2(mut robots: Vec<Robot>) -> Option<usize> {
    let (mut min_non_symmetric, mut t_non_symmetric) = (robots.len(), 0);
    for i in 0..101 * 103 {
        let non_symmetric = count_non_symmetric(&robots);
        if non_symmetric < min_non_symmetric {
            min_non_symmetric = non_symmetric;
            t_non_symmetric = i;
        }
        robots = robots.into_iter().map(|r| r.evolved_by(1)).collect();
    }
    Some(t_non_symmetric)
}

fn count_non_symmetric(robots: &[Robot]) -> usize {
    // Assumption: A christmas tree has symmetry around some vertical line

    // first: decide center by repeatedly taking the average of a contracting set
    let mut x_center = 0;
    for n in [500, 350, 200] {
        x_center = (robots
            .iter()
            .map(|r| r.pos[0])
            .sorted_by_key(|x| (x - x_center).abs())
            .take(n)
            .map(|x| x as f64)
            .sum::<f64>()
            / n as f64)
            .round() as i16;
    }

    // take only robots that are pretty close to the center
    let xs_by_y = robots
        .iter()
        .map(|r| r.pos)
        .filter(|p| (p[0] - x_center).abs() < 10)
        .map(|p| (p[1], p[0] - x_center))
        .acc_into_vec();

    let mut non_symmetric = 0;
    for xs in xs_by_y.into_values() {
        for x in xs.iter() {
            if !xs.contains(&-x) {
                non_symmetric += 1;
            }
        }
    }
    non_symmetric
}

#[derive(Debug)]
struct Robot {
    pos: Pos,
    vel: Vel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Quadrant {
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl Robot {
    fn new(pos: Pos, vel: Vel) -> Robot {
        Robot { pos, vel }
    }

    fn new_constrained(pos: Pos, vel: Vel, max: Pos) -> Robot {
        fn constrained(pos: Pos, max: Pos) -> Pos {
            // place c into the [0, m) interval
            fn constrain(c: Coord, m: Coord) -> Coord {
                ((c % m) + m) % m
            }
            [constrain(pos[0], max[0]), constrain(pos[1], max[1])]
        }
        Self::new(constrained(pos, max), vel)
    }

    fn evolved_by(&self, t: Coord) -> Robot {
        fn evolve(p: Coord, v: Coord, t: Coord) -> Coord {
            t.checked_mul(v).and_then(|d| d.checked_add(p)).unwrap()
        }
        let [px, py] = self.pos;
        let [vx, vy] = self.vel;
        // We don't have to make sure we're in the field after every step
        // (as long as we don't get overflow problems)
        Self::new_constrained([evolve(px, vx, t), evolve(py, vy, t)], self.vel, EXTENT)
    }
}

fn quadrant(p: Pos) -> Option<Quadrant> {
    match (p[0] - EXTENT[0] / 2, p[1] - EXTENT[1] / 2) {
        (x, y) if x > 0 && y < 0 => Some(Quadrant::NorthEast),
        (x, y) if x < 0 && y < 0 => Some(Quadrant::NorthWest),
        (x, y) if x < 0 && y > 0 => Some(Quadrant::SouthWest),
        (x, y) if x > 0 && y > 0 => Some(Quadrant::SouthEast),
        _ => None,
    }
}

impl FromStr for Robot {
    type Err = RobotParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (p, v) = s.split_once(' ').ok_or(RobotParseError::WrongFormat)?;
        let (px, py): (Coord, Coord) = p
            .strip_prefix("p=")
            .and_then(|s| s.split_once(','))
            .ok_or(RobotParseError::WrongFormat)
            .and_then(|(x, y)| Ok((x.parse()?, y.parse()?)))?;
        let (vx, vy): (Coord, Coord) = v
            .strip_prefix("v=")
            .and_then(|s| s.split_once(','))
            .ok_or(RobotParseError::WrongFormat)
            .and_then(|(x, y)| Ok((x.parse()?, y.parse()?)))?;
        Ok(Robot::new([px, py], [vx, vy]))
    }
}

trait AccumulateExt: Iterator {
    fn accumulated_with<KeyFn, AccFn, K, U>(self, key: KeyFn, acc: AccFn) -> HashMap<K, U>
    where
        Self::Item: Clone,
        K: Eq + Hash,
        U: Default,
        KeyFn: Fn(Self::Item) -> K,
        AccFn: Fn(Self::Item, &mut U),
        Self: Sized,
    {
        let mut accumulator = HashMap::new();
        for item in self {
            acc(
                item.clone(),
                accumulator.entry(key(item.clone())).or_default(),
            );
        }
        accumulator
    }
}

impl<I: Iterator> AccumulateExt for I {}

trait CountedExt: Iterator {
    fn counted(self) -> HashMap<Self::Item, usize>
    where
        Self: Sized,
        Self::Item: Clone + Eq + Hash,
    {
        self.accumulated_with(|k| k, |_, c| *c += 1)
    }
}

impl<I: Iterator> CountedExt for I {}

trait AccIntoVecExt: Iterator {
    fn acc_into_vec<K, V>(self) -> HashMap<K, Vec<V>>
    where
        Self: Sized,
        Self::Item: Clone + Into<(K, V)>,
        K: Eq + Hash,
    {
        self.accumulated_with(
            |item| item.into().0,
            |item, items: &mut Vec<_>| items.push(item.into().1),
        )
    }
}

impl<I: Iterator> AccIntoVecExt for I {}

#[derive(Error, Debug)]
enum RobotParseError {
    #[error("Wrong Format")]
    WrongNumberFormat(#[from] ParseIntError),
    #[error("Wrong Format")]
    WrongFormat,
}

// thanks claude
#[allow(unused)]
fn show(robots: &[Robot]) {
    const WIDTH: usize = 101;
    const HEIGHT: usize = 103;

    // Create a 2D grid to track robot positions
    let mut grid = vec![vec![false; WIDTH]; HEIGHT];

    // Mark positions where robots are located
    for robot in robots {
        let x = robot.pos[0] as usize;
        let y = robot.pos[1] as usize;

        // Ensure coordinates are within bounds
        if x < WIDTH && y < HEIGHT {
            grid[y][x] = true;
        }
    }

    // Print the grid
    for row in &grid {
        for &has_robot in row {
            if has_robot {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!(); // New line after each row
    }
}
