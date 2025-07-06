// Points:
// - positive y-direction????????
// - detailed asymptotic analysis notwithstanding,
//   all this overengineering results in only
//   log-costs per guard-turn
// - the 'functional' setup allows of trivial parallelization

use itertools::Itertools;
use rayon::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use tailcall::tailcall;

pub fn solve(input: &str) -> (usize, usize) {
    let (field, guard) = read(&input);

    (part1(&field, guard), part2(&field, guard))
}

fn read(input: &str) -> (Field, Guard) {
    (
        Field {
            obstacles: filter_map_positions(input, |(x, y), c| match c {
                '#' => Some((x as i32, y as i32)),
                _ => None,
            })
            .collect(),
            size: filter_map_positions(input, |(x, y), _| Some((x, y)))
                .last()
                .unwrap(),
        },
        filter_map_positions(input, |(x, y), c| match c {
            '^' => Some(Guard {
                pos: (x, y),
                dir: Direction::Up,
            }),
            '>' => Some(Guard {
                pos: (x, y),
                dir: Direction::Right,
            }),
            '<' => Some(Guard {
                pos: (x, y),
                dir: Direction::Left,
            }),
            'v' => Some(Guard {
                pos: (x, y),
                dir: Direction::Down,
            }),
            _ => None,
        })
        .last()
        .unwrap(),
    )
}

fn filter_map_positions<'a, T, F>(input: &'a str, mut f: F) -> impl Iterator<Item = T> + 'a
where
    F: FnMut(Position, char) -> Option<T> + Copy + 'a + 'static,
{
    input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| f((x as i32, y as i32), c))
        })
        .filter_map(|i| i)
}

fn part1(field: &Field, start: Guard) -> usize {
    std::iter::successors(Some(start), |g| field.step(g.clone()))
        .map(|g| g.pos)
        .collect::<HashSet<Position>>()
        .len()
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn clockwise(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

fn counter_clockwise(dir: Direction) -> Direction {
    clockwise(clockwise(clockwise(dir)))
}

type Position = (i32, i32);

#[derive(Copy, Clone, Eq, PartialEq)]
struct Guard {
    pos: Position,
    dir: Direction,
}

impl Guard {
    fn forward(mut self) -> Guard {
        self.pos = match self.dir {
            Direction::Right => (self.pos.0 + 1, self.pos.1),
            Direction::Left => (self.pos.0 - 1, self.pos.1),
            Direction::Up => (self.pos.0, self.pos.1 - 1),
            Direction::Down => (self.pos.0, self.pos.1 + 1),
        };
        self
    }

    fn back(mut self) -> Guard {
        self.pos = match self.dir {
            Direction::Right => (self.pos.0 - 1, self.pos.1),
            Direction::Left => (self.pos.0 + 1, self.pos.1),
            Direction::Up => (self.pos.0, self.pos.1 + 1),
            Direction::Down => (self.pos.0, self.pos.1 - 1),
        };
        self
    }

    fn turn(mut self) -> Guard {
        self.dir = clockwise(self.dir);
        self
    }
}

fn bound(guard: Guard, bounds: &(i32, i32)) -> Option<Guard> {
    match guard.pos {
        (x, _) if (x < 0) || (x > bounds.0) => None,
        (_, y) if (y < 0) || (y > bounds.1) => None,
        _ => Some(guard),
    }
}

struct Field {
    obstacles: HashSet<Position>,
    size: (i32, i32),
}

impl Field {
    fn step(&self, guard: Guard) -> Option<Guard> {
        bound(guard.forward(), &self.size).and_then(|guard| {
            if self.obstacles.contains(&guard.pos) {
                Some(guard.back().turn())
            } else {
                Some(guard)
            }
        })
    }
}

fn part2(field: &Field, start: Guard) -> usize {
    // Idea: go stepwise through original walk. At every step, put obstacle in front of guard.
    // Then, start the walk from the beginning with the hypothetical obstacle in place. If at
    // any point during this new walk the path of the guard overlaps (is colinear), then there
    // is a loop.
    // To make this computationally sane, need a structure to efficiently look up lines
    // Also can take advantage of this being very parallelizable

    let mut obstacles = Lines::new();
    field
        .obstacles
        .iter()
        .for_each(|o| obstacles.add_obstacle(*o));

    std::iter::successors(Some(start), |g| field.step(g.clone()))
        .collect::<Vec<_>>()
        .into_par_iter()
        .filter_map(|g| {
            let in_front = g.forward().pos;
            if in_front == start.pos {
                // cannot put obstacle where guard is at currently
                return None;
            }
            let loop_found = would_loop(
                start,
                &obstacles,
                Lines::new().with_obstacle(in_front),
                Lines::new(),
            );
            loop_found.then_some(in_front)
        })
        .collect::<HashSet<Position>>()
        .len()
}

// A line describing a straight path of a guard (so technically a directed Line-Segment)
// Direction of the line, const coordinate in counter-clockwise direction, (start in direction, end in direction)
type Line = (Direction, i32, (i32, i32));

// Line coordinates: first unchanging in counter-clockwise dir, then changing in dir
fn to_line_coordinates(pos: Position, dir: Direction) -> (i32, i32) {
    (
        coordinate_in_direction(pos, counter_clockwise(dir)),
        coordinate_in_direction(pos, dir),
    )
}

fn from_line_coordinates(pos: (i32, i32), dir: Direction) -> Position {
    let (x_unchanging, y_unchanging) = coordinate_from_direction(pos.0, counter_clockwise(dir));
    let (x_changing, y_changing) = coordinate_from_direction(pos.1, dir);
    (x_unchanging + x_changing, y_unchanging + y_changing)
}

fn coordinate_in_direction(pos: Position, dir: Direction) -> i32 {
    let (x, y) = pos;
    match dir {
        Direction::Up => -y,
        Direction::Right => x,
        Direction::Down => y,
        Direction::Left => -x,
    }
}

fn coordinate_from_direction(coord: i32, dir: Direction) -> Position {
    match dir {
        Direction::Up => (0, -coord),
        Direction::Right => (coord, 0),
        Direction::Down => (0, coord),
        Direction::Left => (-coord, 0),
    }
}

fn points_to_line(start: Position, end: Position, dir: Direction) -> Line {
    // TODO: Write this with coordinate_in_direction?
    let ((x1, y1), (x2, y2)) = (start, end);
    if x1 == x2 {
        if y1 > y2 {
            assert!(dir == Direction::Up);
            return (Direction::Up, -x1, (-y1, -y2));
        }
        if y2 > y1 {
            assert!(dir == Direction::Down);
            return (Direction::Down, x1, (y1, y2));
        }
    }
    if y1 == y2 {
        if x1 > x2 {
            assert!(dir == Direction::Left);
            return (Direction::Left, y1, (-x1, -x2));
        }
        if x2 > x1 {
            assert!(dir == Direction::Right);
            return (Direction::Right, -y1, (x1, x2));
        }
    }
    if x1 == x2 && y1 == y2 {
        // line of length 1 is fine
        let (unchanging, changing) = to_line_coordinates(start, dir);
        return (dir, unchanging, (changing, changing));
    }
    panic!("Line must align with grid, and cannot be a single point");
}

// An obstacle can be modeled as 4 Lines
fn point_to_lines(position: Position) -> Vec<Line> {
    let (x, y) = position;
    vec![
        (Direction::Up, -x, (-y, -y)),
        (Direction::Right, -y, (x, x)),
        (Direction::Down, x, (y, y)),
        (Direction::Left, y, (-x, -x)),
    ]
}

#[derive(Clone, Debug)]
struct Intervals {
    // Interval start -> Interval end
    intervals: BTreeMap<i32, i32>,
}

impl Intervals {
    fn new() -> Self {
        Intervals {
            intervals: BTreeMap::new(),
        }
    }

    // x  |-a-| -> Some(a.start)
    // |--a-x---| -> Some(a.start)
    // |-a-| x |--b--| -> Some(b.start)
    // |-a-| x -> None
    fn first_larger_equals(&self, x: i32) -> Option<i32> {
        self.intervals
            .range(..=x)
            .rev()
            .next()
            .filter(|(_, e)| e >= &&x)
            .or_else(|| self.intervals.range(x..).next())
            .map(|(&s, &_)| s)
    }

    fn add(&mut self, start: i32, end: i32) {
        let (start, end) = (start.min(end), start.max(end));

        let touching: Vec<(i32, i32)> = self
            .intervals
            .range(..=start + 1)
            .rev()
            // possibly touching with the first existing interval with a smaller start
            .take(1)
            .filter(|(_, existing_end)| **existing_end + 1 >= start)
            // overlapping with all existing intervals that start inside the new one
            .chain(self.intervals.range(start..=end + 1))
            .map(|(s, e)| (*s, *e))
            .collect();

        if touching.is_empty() {
            self.intervals.insert(start, end);
            return;
        }

        let min_start = touching.iter().map(|(s, _)| *s).min().unwrap().min(start);
        let max_end = touching.iter().map(|(_, e)| *e).max().unwrap().max(end);

        for (start, _) in touching {
            self.intervals.remove(&start);
        }

        self.intervals.insert(min_start, max_end);
    }
}

#[derive(Clone, Debug)]
struct Lines {
    // line direction -> (const coord -> (start -> end))
    lines: HashMap<Direction, BTreeMap<i32, Intervals>>,
}

impl Lines {
    fn new() -> Lines {
        Lines {
            lines: HashMap::new(),
        }
    }

    fn with_obstacle(&self, position: Position) -> Lines {
        let mut new = Lines {
            lines: self.lines.clone(),
        };
        new.add_obstacle(position);
        new
    }

    fn add_obstacle(&mut self, position: Position) {
        point_to_lines(position)
            .iter()
            .for_each(|&l| self.add_line(l));
    }

    // because we want to be able to add 1-length linesegments, we also specify dir.
    // however, this interface is not so nice because information is duplicated (which in turn means
    // a partial function / interdependence between function arguments)
    fn add_points_as_line(&mut self, line_start: Position, line_end: Position, dir: Direction) {
        self.add_line(points_to_line(line_start, line_end, dir));
    }

    fn add_line(&mut self, line: Line) {
        let (dir, const_coord, (start, end)) = line;

        // See simpler entry example: https://doc.rust-lang.org/std/collections/btree_map/struct.BTreeMap.html#method.entry
        self.lines
            .entry(dir)
            .or_insert(BTreeMap::new())
            .entry(const_coord)
            .or_insert(Intervals::new())
            .add(start, end);
    }

    // return coordinate (line-coords) of the start of first colinear line
    fn first_colinear(&self, start: Position, dir: Direction) -> Option<(i32, i32)> {
        let (unchanging, changing) = to_line_coordinates(start, dir);
        self.lines
            .get(&dir)
            .and_then(|lines_in_dir| lines_in_dir.get(&unchanging))
            .and_then(|intervals| intervals.first_larger_equals(changing))
            .map(|start| (unchanging, start.max(changing)))
    }
}

// see https://stackoverflow.com/q/59257543 etc
// also check out what happens if the recursive call is replaced with
// would_loop(...) == true
#[tailcall]
// Would this setup result in a loop?
fn would_loop(
    guard: Guard,
    fixed_obstacles: &Lines,
    hypothetical_obstacle: Lines,
    mut history: Lines,
) -> bool {
    // Take the hypthetical_obstacle as seperate Lines, to prevent having to repeatedly copy the original
    // obstacles and inserting just one other obstacle.
    // Then look through both and take the closer collision
    let next_obstacle = [fixed_obstacles, &hypothetical_obstacle]
        .iter()
        .filter_map(|obstacles| obstacles.first_colinear(guard.pos, guard.dir))
        .sorted_by_key(|next_obstacle| next_obstacle.1)
        .next();
    match (next_obstacle, history.first_colinear(guard.pos, guard.dir)) {
        (None, Some(_)) => true,
        (Some((_, obstacle_dist)), Some((_, overlap_dist))) if obstacle_dist > overlap_dist => true,
        (Some(obstacle), _) => {
            let turn_at = from_line_coordinates((obstacle.0, obstacle.1 - 1), guard.dir);
            history.add_points_as_line(guard.pos, turn_at, guard.dir);
            let new_guard = Guard {
                pos: turn_at,
                dir: clockwise(guard.dir),
            };
            would_loop(new_guard, fixed_obstacles, hypothetical_obstacle, history)
        }
        _ => false,
    }
}

// Thanks Claude
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intervals() {
        fn get_all(i: &Intervals) -> Vec<(i32, i32)> {
            i.intervals.iter().map(|(&k, &v)| (k, v)).collect()
        }
        let mut intervals = Intervals::new();

        // Test case 1: No overlap
        intervals.add(1, 3);
        intervals.add(6, 8);
        assert_eq!(get_all(&intervals), vec![(1, 3), (6, 8)]);

        // Test case 2: Overlap at end
        intervals.add(3, 6);
        assert_eq!(get_all(&intervals), vec![(1, 8)]);

        // Test case 3: Completely contained
        intervals.add(2, 4);
        assert_eq!(get_all(&intervals), vec![(1, 8)]);

        // Test case 4: Overlap multiple intervals
        intervals.add(10, 12);
        intervals.add(7, 11);
        assert_eq!(get_all(&intervals), vec![(1, 12)]);

        // Test case 5: No overlap with reversed start/end
        intervals.add(15, 14);
        assert_eq!(get_all(&intervals), vec![(1, 12), (14, 15)]);

        // adjacent should merge
        intervals.add(16, 17);
        assert_eq!(get_all(&intervals), vec![(1, 12), (14, 17)]);
    }

    #[test]
    fn test_coordinate_transforms_are_inverses() {
        // Test positions to check - include origin, positive/negative coordinates, and larger numbers
        let test_positions: Vec<Position> = vec![
            (0, 0),
            (1, 0),
            (0, 1),
            (-1, 0),
            (0, -1),
            (1, 1),
            (-1, -1),
            (5, -3),
            (-2, 7),
            (10, 10),
        ];

        let directions = [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];

        for &pos in &test_positions {
            for &dir in &directions {
                // Test that from_line_coordinates(to_line_coordinates(pos)) == pos
                let line_coords = to_line_coordinates(pos, dir);
                let recovered_pos = from_line_coordinates(line_coords, dir);
                assert_eq!(
                    pos, recovered_pos,
                    "Failed to recover original position {:?} after converting to line coordinates {:?} with direction {:?}",
                    pos, line_coords, dir
                );

                // Test that to_line_coordinates(from_line_coordinates(line_coords)) == line_coords
                let new_pos = from_line_coordinates(line_coords, dir);
                let recovered_line_coords = to_line_coordinates(new_pos, dir);
                assert_eq!(
                    line_coords, recovered_line_coords,
                    "Failed to recover line coordinates {:?} after converting to position {:?} with direction {:?}",
                    line_coords, new_pos, dir
                );
            }
        }
    }
}
