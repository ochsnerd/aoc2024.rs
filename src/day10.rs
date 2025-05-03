use std::{collections::HashSet, fs};

pub fn day10(input_path: String) {
    let content = fs::read_to_string(input_path).unwrap();

    let map = Map::new(&content);

    println!("{:?}", part1(&map));
    println!("{:?}", part2(&map));
}

fn part1(map: &Map) -> usize {
    map.iter()
        // These are the trailheads
        .filter_map(|(p, v)| if v == 0 { Some(p) } else { None })
        .map(|p| map.reachable_from(p).filter(|&p| map.at(p) == 9).count())
        .sum()
}

fn part2(map: &Map) -> usize {
    map.iter()
        // These are the trailheads
        .filter_map(|(p, v)| if v == 0 { Some(p) } else { None })
        .map(|p| map.all_paths_from(p).filter(|&p| map.at(p) == 9).count())
        .sum()
}

type Position = (usize, usize);

#[derive(Debug)]
struct Map {
    data: Vec<Vec<u32>>,
    max: Position,
}

impl Map {
    fn new(s: &str) -> Map {
        let data: Vec<Vec<_>> = s
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();

        // 0-size Maps are disallowed
        let max = (data.len(), data[0].len());

        data.iter().for_each(|l| {
            if l.len() != max.1 {
                panic!();
            }
        });
        Map { data, max }
    }

    fn at(&self, p: Position) -> u32 {
        self.data[p.0][p.1]
    }

    fn neighbors(&self, p: Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        if p.0 > 0 {
            neighbors.push((p.0 - 1, p.1));
        }
        if p.0 < self.max.0 - 1 {
            neighbors.push((p.0 + 1, p.1))
        }
        if p.1 > 0 {
            neighbors.push((p.0, p.1 - 1));
        }
        if p.1 < self.max.1 - 1 {
            neighbors.push((p.0, p.1 + 1))
        }
        neighbors
    }

    fn reachable_neighbors(&self, p: Position) -> Vec<Position> {
        let here = self.at(p);
        self.neighbors(p)
            .into_iter()
            .filter(|&p| here + 1 == self.at(p))
            .collect()
    }

    fn iter(&self) -> MapIter {
        self.into_iter()
    }

    fn reachable_from(&self, p: Position) -> PathIter {
        PathIter::new(self, p)
    }

    fn all_paths_from(&self, p: Position) -> AllPathIter {
        AllPathIter::new(self, p)
    }
}

impl<'a> IntoIterator for &'a Map {
    type Item = (Position, u32);
    type IntoIter = MapIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MapIter::new(self)
    }
}

struct MapIter<'a> {
    map: &'a Map,
    current: Position,
}

impl<'a> MapIter<'a> {
    fn new(map: &'a Map) -> MapIter<'a> {
        MapIter {
            map,
            current: (0, 0),
        }
    }
}

impl<'a> Iterator for MapIter<'a> {
    type Item = (Position, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let mut next = (self.current.0, self.current.1 + 1);
        if next.1 >= self.map.max.1 {
            next.0 += 1;
            next.1 = 0;
        }
        if next.0 >= self.map.max.0 {
            return None;
        }
        self.current = next;
        Some((self.current, self.map.at(self.current)))
    }
}

struct PathIter<'a> {
    map: &'a Map,
    visited: HashSet<Position>,
    todo: Vec<Position>,
}

impl<'a> PathIter<'a> {
    fn new(map: &'a Map, start: Position) -> PathIter<'a> {
        PathIter {
            map,
            visited: HashSet::new(),
            todo: vec![start],
        }
    }
}

impl<'a> Iterator for PathIter<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(here) = self.todo.pop() {
            if self.visited.insert(here) {
                self.todo
                    .extend_from_slice(&self.map.reachable_neighbors(here));
                return Some(here);
            }
        }
        None
    }
}

struct AllPathIter<'a> {
    map: &'a Map,
    todo: Vec<Position>,
}

impl<'a> AllPathIter<'a> {
    fn new(map: &'a Map, start: Position) -> AllPathIter<'a> {
        AllPathIter {
            map,
            todo: vec![start],
        }
    }
}

impl<'a> Iterator for AllPathIter<'a> {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(here) = self.todo.pop() {
            self.todo
                .extend_from_slice(&self.map.reachable_neighbors(here));
            return Some(here);
        }
        None
    }
}
