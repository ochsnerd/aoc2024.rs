use std::{
    collections::HashMap,
    hash::Hash,
    num::TryFromIntError,
    ops::{Add, AddAssign, IndexMut},
};

use itertools::{IntoChunks, Itertools};

pub type Size = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Index((usize, usize));

pub fn monadic(size: Size, index: Index) -> usize {
    index.0 .0 + index.0 .1 * size.0
}

pub fn dyadic(size: Size, index: usize) -> Index {
    Index((index % size.0, index / size.0))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Into<Vector> for Direction {
    fn into(self) -> Vector {
        match self {
            Direction::Up => Vector::new([0, 1]),
            Direction::Right => Vector::new([1, 0]),
            Direction::Down => Vector::new([0, -1]),
            Direction::Left => Vector::new([-1, 0]),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub size: Size,
    pub elements: Vec<T>,
}

impl<T> Grid<T> {
    pub fn new(size: Size, elements: Vec<T>) -> Grid<T> {
        if size.0 * size.1 != elements.len() {
            panic!("Misatched size");
        }
        Grid { size, elements }
    }

    pub fn at(&self, idx: Index) -> Option<&T> {
        if idx.0 .0 < self.size.0 && idx.0 .1 < self.size.1 {
            Some(&self[idx])
        } else {
            None
        }
    }

    pub fn at_point(&self, pos: Point) -> Option<&T> {
        pos.try_into().ok().and_then(|idx: Index| self.at(idx))
    }

    pub fn swap(&mut self, a: Index, b: Index) {
        self.elements
            .swap(monadic(self.size, a), monadic(self.size, b));
    }

    pub fn iter_indices(&self) -> impl Iterator<Item = Index> {
        let size = self.size;
        (0..self.size.0 * self.size.1).map(move |i| dyadic(size, i))
    }

    pub fn iter_indices_by_rows(&self) -> IntoChunks<impl Iterator<Item = Index>> {
        let rows = self.size.0;
        self.iter_indices().chunks(rows)
    }
}

pub struct GridParser<T> {
    mapping: HashMap<char, T>,
}

impl<T: Copy, const N: usize> From<[(char, T); N]> for GridParser<T> {
    fn from(arr: [(char, T); N]) -> Self {
        Self::new(arr.into())
    }
}

#[derive(Debug)]
pub struct GridParseError;

impl<T: Copy> GridParser<T> {
    pub fn new(mapping: HashMap<char, T>) -> Self {
        Self { mapping }
    }

    pub fn parse(self, s: &str) -> Result<Grid<T>, GridParseError> {
        let lines: Vec<_> = s.lines().collect();
        if lines.is_empty() {
            return Ok(Grid::new((0, 0), Vec::new()));
        }
        let size = (lines[0].len(), lines.len());
        let things: Option<Vec<_>> = lines
            .into_iter()
            .flat_map(|v| v.chars())
            .map(|c| self.mapping.get(&c).map(|t| *t))
            .collect();

        match things {
            Some(things) => Ok(Grid::new(size, things)),
            None => Err(GridParseError),
        }
    }
}

impl<T> std::ops::Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}

impl<T> IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[index]
    }
}

impl<T> std::ops::Index<Index> for Grid<T> {
    type Output = T;

    fn index(&self, index: Index) -> &Self::Output {
        &self[monadic(self.size, index)]
    }
}

impl<T> IndexMut<Index> for Grid<T> {
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        let idx = monadic(self.size, index);
        &mut self[idx]
    }
}

pub type Signed = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point(Coords);

#[derive(Debug, Clone, Copy)]
pub struct Vector(Coords);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coords([Signed; 2]);

impl Point {
    pub fn new(coords: [Signed; 2]) -> Self {
        Point(Coords(coords))
    }

    pub fn scaled_x(self, factor: Signed) -> Self {
        Self::new([self.0 .0[0] * factor, self.0 .0[1]])
    }
}

impl Vector {
    fn new(coords: [Signed; 2]) -> Self {
        Vector(Coords(coords))
    }
}

impl TryFrom<Index> for Coords {
    type Error = TryFromIntError;
    fn try_from(item: Index) -> Result<Self, Self::Error> {
        Ok(Coords([item.0 .0.try_into()?, -item.0 .1.try_into()?]))
    }
}

impl TryFrom<Coords> for Index {
    type Error = TryFromIntError;
    fn try_from(item: Coords) -> Result<Self, Self::Error> {
        Ok(Index((item.0[0].try_into()?, (-item.0[1]).try_into()?)))
    }
}

impl TryFrom<Point> for Index {
    type Error = TryFromIntError;
    fn try_from(value: Point) -> Result<Self, Self::Error> {
        Ok(value.0.try_into()?)
    }
}

impl TryFrom<Index> for Point {
    type Error = TryFromIntError;
    fn try_from(value: Index) -> Result<Self, Self::Error> {
        Ok(Point(value.try_into()?))
    }
}

impl From<Index> for (usize, usize) {
    fn from(value: Index) -> Self {
        value.0
    }
}

impl AddAssign<Vector> for Point {
    fn add_assign(&mut self, rhs: Vector) {
        self.0 += rhs.0;
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.0 += rhs.0;
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(mut self, other: Vector) -> Self {
        self.0 += other.0;
        self
    }
}

impl AddAssign for Coords {
    fn add_assign(&mut self, rhs: Coords) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
    }
}

impl Add for Coords {
    type Output = Coords;
    fn add(mut self, other: Coords) -> Self {
        self += other;
        self
    }
}
