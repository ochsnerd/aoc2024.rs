use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

pub fn day06(mut input_path: PathBuf) {
    input_path.push("06_test.txt");
    let content = fs::read_to_string(input_path).unwrap();

    let (field, guard) = read(&content);

    println!(
        "{:?}",
        lines_from_obstacles(&field.obstacles).vertical_lines
    );
    println!(
        "{:?}",
        lines_from_obstacles(&field.obstacles).horizontal_lines
    );
    println!("{:?}", part1(&field, guard));
    println!("{:?}", part2(&field, guard));
}

fn part1(field: &Field, start: Guard) -> usize {
    std::iter::successors(Some(start), |g| field.step(g.clone()))
        .map(|g| g.pos)
        .collect::<HashSet<Position>>()
        .len()
}

fn part2(field: &Field, start: Guard) -> usize {
    let (_, b) = std::iter::successors(Some(start), |g| field.step(g.clone())).fold(
        (HashMap::<Position, Guard>::new(), Vec::<Position>::new()),
        |(mut visited, mut blockages), guard| {
            if let Some(last_time) = visited.get(&guard.pos) {
                if &guard.turn() == last_time {
                    blockages.push(guard.forward().pos);
                }
            }
            visited.insert(guard.pos, guard);
            (visited, blockages)
        },
    );

    b.len()
}

// TODO: Linescan
// fn:  loop-aware walk at P, facing D, with Os (Obstacles (TODO: How does this look?)), Hs (History (HashMap<Direction, HashMap<i32, Vec<i32>>>)):
//   - Check in Hs of D+1 what the closest lineend is (incl direction: larger (if right, up)  or smaller (if left, down))
//   - Check in Os what the closest obstacle is
//   - if lineend is closest -> Loop
//   - if obstacle is closest -> loop-aware walk at O - D, facing D + 1
//   - if nothing is closest -> Done

// Fold over Iter<Guard>, accumulate Hs, usize Loops
// |(mut Hs, mut n_loops), guard| {
//    if loop-aware walk guard.pos, guard.dir == Loop {loops += 1;}
//    update Hs with current pos (either += 1 existing lineend, or add new one (efficiently?))
//    (Hs, n_loops)
// }

struct Lines {
    // Mapping from <unchanging_coordinate> -> (<start> -> <length>)
    // Line from (1, 1) to (4, 1) is horizontal_lines[1][1] == 4
    // Line from (3, 2) to (3, 3) is vertical_lines[3][1] == 2
    // If two lines are colinear and with the same starting point, the longer of the two is stored
    vertical_lines: HashMap<i32, BTreeMap<i32, i32>>,
    horizontal_lines: HashMap<i32, BTreeMap<i32, i32>>,
}

impl Lines {
    fn new() -> Lines {
	Lines {
	    vertical_lines: HashMap::new(),
	    horizontal_lines: HashMap::new(),
	}
    }
    fn add(&mut self, line: (Position, Position))
    {
	let ((x1, y1), (x2, y2)) = line;
	
        if x1 == x2 {
            // See simpler entry example: https://doc.rust-lang.org/std/collections/btree_map/struct.BTreeMap.html#method.entry
            let start = y1.min(y2);
            let end = y1.max(y2);
	    self.vertical_lines
                .entry(x1)
                .or_insert(BTreeMap::new())
                .entry(start)
                .and_modify(|e| *e = (*e).max(end))
                .or_insert(end);
        }
        if y1 == y2 {
            let start = x1.min(x2);
            let end = x1.max(x2);
	    self.horizontal_lines
                .entry(y1)
                .or_insert(BTreeMap::new())
                .entry(start)
                .and_modify(|e| *e = (*e).max(end))
                .or_insert(end);
        }
    }
    fn add_step(&mut self, guard: Guard)
    {
    }
}

impl FromIterator<(Position, Position)> for Lines
{
    fn from_iter<T: IntoIterator<Item = (Position, Position)>>(iter: T) -> Self {
	let mut lines = Lines::new();
	for l in iter {
	    lines.add(l);
	}
	lines
    }
}

fn lines_from_obstacles(obstacles: &HashSet<Position>) -> Lines {
    // obstacles are just lines of length one
    obstacles.iter().map(|&p| (p, p)).collect()
}

fn read(input: &str) -> (Field, Guard) {
    (
        Field {
            obstacles: filter_map_positions(input, |(x, y), c| match c {
                '#' => Some((x as i32, y as i32)),
                _ => None,
            })
            .collect(),
            size: filter_map_positions(input, |(x, y), _| Some((x as i32, y as i32)))
                .last()
                .unwrap(),
        },
        filter_map_positions(input, |(x, y), c| match c {
            '^' => Some(Guard {
                pos: (x as i32, y as i32),
                dir: Direction::Up,
            }),
            '>' => Some(Guard {
                pos: (x as i32, y as i32),
                dir: Direction::Right,
            }),
            '<' => Some(Guard {
                pos: (x as i32, y as i32),
                dir: Direction::Left,
            }),
            'v' => Some(Guard {
                pos: (x as i32, y as i32),
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

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
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
        self.dir = match self.dir {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
        self
    }
}

fn bound(guard: Guard, bounds: &Position) -> Option<Guard> {
    match guard.pos {
        (x, _) if (x < 0) || (x > bounds.0) => None,
        (_, y) if (y < 0) || (y > bounds.1) => None,
        _ => Some(guard),
    }
}

struct Field {
    obstacles: HashSet<Position>,
    size: Position,
}

impl Field {
    fn step(&self, guard: Guard) -> Option<Guard> {
        bound(guard.forward(), &self.size).and_then(|guard| {
            if self.obstacles.contains(&guard.pos) {
                self.step(guard.back().turn())
            } else {
                Some(guard)
            }
        })
    }
}

// this was then rewriteen as Impl FromIterator<Lines>
// fn collect_into_lines<I>(coordinates: I) -> Lines
// where
//     I: IntoIterator<Item = (Position, Position)>,
// {
//     let mut v_lines: HashMap<i32, BTreeMap<i32, i32>> = HashMap::new();
//     let mut h_lines: HashMap<i32, BTreeMap<i32, i32>> = HashMap::new();
//     for ((x1, y1), (x2, y2)) in coordinates {
//         if x1 == x2 {
//             // See simpler entry example: https://doc.rust-lang.org/std/collections/btree_map/struct.BTreeMap.html#method.entry
//             let start = y1.min(y2);
//             let end = y1.max(y2);
//             v_lines
//                 .entry(x1)
//                 .or_insert(BTreeMap::new())
//                 .entry(start)
//                 .and_modify(|e| *e = (*e).max(end))
//                 .or_insert(end);
//         }
//         if y1 == y2 {
//             let start = x1.min(x2);
//             let end = x1.max(x2);
//             h_lines
//                 .entry(y1)
//                 .or_insert(BTreeMap::new())
//                 .entry(start)
//                 .and_modify(|e| *e = (*e).max(end))
//                 .or_insert(end);
//         }
//     }

//     Lines {
//         vertical_lines: v_lines,
//         horizontal_lines: h_lines,
//     }
// }
