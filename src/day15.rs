use core::fmt;

use crate::{
    graph::bfs,
    grid::{Direction, Grid, GridParseError, GridParser, Index, Point, Vector},
};

pub fn solve(input: &str) -> (usize, usize) {
    let (warehouse, movements) = parse(input);
    let warehouse2 = warehouse.make_part2();
    (part1(warehouse, &movements), part2(warehouse2, &movements))
}

fn part1(mut warehouse: Warehouse, movements: &[Direction]) -> usize {
    for &m in movements.into_iter() {
        warehouse.do_move(m)
    }
    warehouse.score()
}

fn part2(mut warehouse: Warehouse2, movements: &[Direction]) -> usize {
    for &m in movements.into_iter() {
        warehouse.do_move(m)
    }
    warehouse.score()
}

fn score(i: Index) -> usize {
    let (x, y) = i.into();
    x + y * 100
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Thing {
    Wall,
    Box,
    Floor,
}

#[derive(Debug, Clone)]
struct Warehouse {
    robot: Point,
    map: Grid<Thing>,
}

impl Warehouse {
    fn new(plan: &str) -> Warehouse {
        // Here we iterate twice over plan - this not cost a meaningful amount of time
        let map = GridParser::new(|c| match c {
            '#' => Ok(Thing::Wall),
            '.' => Ok(Thing::Floor),
            'O' => Ok(Thing::Box),
            '@' => Ok(Thing::Floor),
            _ => Err(GridParseError),
        })
        .parse(plan)
        .unwrap();
        Warehouse {
            robot: plan
                .lines()
                .flat_map(|l| l.chars())
                .enumerate()
                .filter_map(|(i, c)| match c {
                    '@' => Some(map.make_index(i).try_into().unwrap()),
                    _ => None,
                })
                .next()
                .unwrap(),
            map,
        }
    }

    fn do_move(&mut self, movement: Direction) {
        let step = movement.into();
        let start = self.robot + step;
        let mut end = start;

        while self.map.at_point(end) == Some(&Thing::Box) {
            end += step;
        }

        if self.map.at_point(end) == Some(&Thing::Wall) {
            // blocked
            return;
        }

        self.robot = start;
        self.map
            .swap(start.try_into().unwrap(), end.try_into().unwrap());
    }

    fn score(&self) -> usize {
        self.map
            .iter_indices()
            .filter(|&i| self.map[i] == Thing::Box)
            .map(score)
            .sum()
    }

    fn make_part2(&self) -> Warehouse2 {
        Warehouse2 {
            robot: self.robot.scaled_x(2),
            map: Grid::new(
                (self.map.size.0 * 2, self.map.size.1),
                self.map
                    .elements
                    .iter()
                    .flat_map(|t| match t {
                        Thing::Wall => [Thing2::Wall, Thing2::Wall],
                        Thing::Box => [Thing2::BoxLeft, Thing2::BoxRight],
                        Thing::Floor => [Thing2::Floor, Thing2::Floor],
                    })
                    .collect(),
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Thing2 {
    Wall,
    BoxLeft,
    BoxRight,
    Floor,
}

fn is_box(t: &Thing2) -> bool {
    t == &Thing2::BoxLeft || t == &Thing2::BoxRight
}

#[derive(Debug)]
struct Warehouse2 {
    robot: Point,
    map: Grid<Thing2>,
}

impl Warehouse2 {
    fn do_move(&mut self, movement: Direction) {
        let step = movement.into();
        self.push(self.robot, step);
        if self.map.at_point(self.robot + step) == Some(&Thing2::Floor) {
            self.robot += step;
        }
    }

    fn push(&mut self, pos: Point, step: Vector) {
        let pushes = |p| {
            let next = p + step;
            if self.map.at_point(next).is_some_and(is_box) {
                vec![next, Warehouse2::other_box(&self.map, next).unwrap()]
            } else {
                Vec::new()
            }
        };

        let potentially_pushed: Vec<_> = bfs(vec![pos], |&p| pushes(p)).skip(1).collect();

        if potentially_pushed
            .iter()
            .any(|&p| self.map.at_point(p + step) == Some(&Thing2::Wall))
        {
            return;
        }

        // note: rev works here because we did a bfs
        for p in potentially_pushed.into_iter().rev() {
            self.map
                .swap(p.try_into().unwrap(), (p + step).try_into().unwrap());
        }
    }

    fn other_box(map: &Grid<Thing2>, p: Point) -> Option<Point> {
        match map.at_point(p) {
            Some(&Thing2::BoxLeft) => Some(p + Direction::Right.into()),
            Some(&Thing2::BoxRight) => Some(p + Direction::Left.into()),
            _ => None,
        }
    }

    fn score(&self) -> usize {
        self.map
            .iter_indices()
            .filter(|&i| self.map[i] == Thing2::BoxLeft)
            .map(score)
            .sum()
    }
}

// no error handling ¯\_(ツ)_/¯
fn parse(input: &str) -> (Warehouse, Vec<Direction>) {
    fn from_arrow(a: char) -> Direction {
        match a {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!("Not an arrow"),
        }
    }
    let mut blocks = input.split("\n\n");
    if let (Some(warehouse_plan), Some(movements)) = (blocks.next(), blocks.next()) {
        return (
            Warehouse::new(warehouse_plan),
            movements
                .lines()
                .flat_map(|l| l.chars().map(from_arrow))
                .collect(),
        );
    }
    panic!("Input looks unexpected");
}

impl fmt::Display for Warehouse2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let robot = self.robot.try_into().unwrap();
        write!(
            f,
            "{}",
            self.map.display(|t, i| match (t, i) {
                (_, i) if i == robot => '@',
                (Thing2::Wall, _) => '#',
                (Thing2::BoxLeft, _) => '[',
                (Thing2::BoxRight, _) => ']',
                (Thing2::Floor, _) => '.',
            })
        )?;

        Ok(())
    }
}
