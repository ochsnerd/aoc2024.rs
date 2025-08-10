use itertools::Itertools;

use crate::{
    graph::{dijkstra, dijkstra_all},
    grid::{Direction, Grid, GridParseError, GridParser, Index},
    util::IteratorExt,
};

pub fn solve(input: &str) -> (usize, usize) {
    let (start, end, map) = parse(input).unwrap();

    // println!(
    //     "{}",
    //     map.display(
    //         |t, _| match t {
    //             Thing::Wall => '#',
    //             Thing::Floor => '.',
    //         }
    //     )
    // );

    let (end_dir, cost) = part1(start.clone(), end, &map);
    let count_on_best_path = part2(
        start,
        Pose {
            position: end,
            heading: end_dir,
        },
        &map,
    );

    (cost, count_on_best_path)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pose {
    position: Index,
    heading: Direction,
}

impl Ord for Pose {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let pos: (_, _) = self.position.into();
        pos.cmp(&other.position.into())
            .then_with(|| self.heading.cmp(&other.heading))
    }
}

impl PartialOrd for Pose {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum Thing {
    Wall,
    Floor,
}

fn cost(path: &[Pose]) -> usize {
    path.iter()
        .tuple_windows()
        .map(|(p1, p2)| if p1.heading != p2.heading { 1000 } else { 1 })
        .sum()
}

fn part1(start: Pose, end: Index, map: &Grid<Thing>) -> (Direction, usize) {
    // need to check all 4 possible end-headings
    Direction::all()
        .iter()
        .map(|&d| Pose {
            position: end,
            heading: d,
        })
        .map(|e| {
            (
                e.heading,
                dijkstra(start.clone(), |p| movements(p, map), e)
                    .unwrap() // we're sure there's a way to the end
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(dir, path)| (dir, cost(&path)))
        .min_by_key(|(_, cost)| *cost)
        .unwrap() // we know the iterator is not empty
}

fn movements(pose: &Pose, map: &Grid<Thing>) -> impl IntoIterator<Item = (usize, Pose)> {
    let straight = pose.position.neighbor(pose.heading);
    [
        map.at(straight).and_then(|t| match t {
            Thing::Floor => Some((
                1,
                Pose {
                    position: straight,
                    heading: pose.heading,
                },
            )),
            Thing::Wall => None,
        }),
        Some((
            1000,
            Pose {
                position: pose.position,
                heading: pose.heading.clockwise(),
            },
        )),
        Some((
            1000,
            Pose {
                position: pose.position,
                heading: pose.heading.anti_clockwise(),
            },
        )),
    ]
    .into_iter()
    .filter_map(|p| p)
}

fn part2(start: Pose, end: Pose, map: &Grid<Thing>) -> usize {
    dijkstra_all(start, |p| movements(p, map), end)
        .into_iter()
        .flatten()
        .map(|p| p.position)
        .uniques()
        .count()
}

fn parse(input: &str) -> Result<(Pose, Index, Grid<Thing>), GridParseError> {
    let map = GridParser::new(|c| match c {
        '#' => Ok(Thing::Wall),
        '.' => Ok(Thing::Floor),
        'S' => Ok(Thing::Floor),
        'E' => Ok(Thing::Floor),
        _ => Err(GridParseError),
    })
    .parse(input)?;

    let start = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .enumerate()
        .filter_map(|(i, c)| match c {
            'S' => Some(map.make_index(i)),
            _ => None,
        })
        .next()
        .ok_or(GridParseError)?;

    let start = Pose {
        position: start,
        heading: Direction::Right,
    };

    let end = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .enumerate()
        .filter_map(|(i, c)| match c {
            'E' => Some(map.make_index(i)),
            _ => None,
        })
        .next()
        .ok_or(GridParseError)?;

    Ok((start, end, map))
}
