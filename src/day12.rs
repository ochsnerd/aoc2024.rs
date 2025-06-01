use std::{
    cmp::minmax_by_key,
    collections::{HashMap, HashSet},
    ops::Index,
};

pub fn day12(input: &str) -> (usize, usize) {
    let garden = Garden::new(input);

    (part1(&garden), part2(&garden))
}

fn part1(garden: &Garden) -> usize {
    garden.iter_regions().map(|r| cost1(&r, &garden)).sum()
}

fn part2(garden: &Garden) -> usize {
    garden.iter_regions().map(|r| cost2(&r, &garden)).sum()
}

fn cost1(region: &HashSet<MonadicIndex>, garden: &Garden) -> usize {
    region
        .iter()
        .map(|&i| fences(i, garden).count())
        .sum::<usize>()
        * region.len()
}

fn cost2(region: &HashSet<MonadicIndex>, garden: &Garden) -> usize {
    // give each fence an index, keep lookup fast
    let all_fences: HashMap<Fence, usize> = region
        .iter()
        .flat_map(|&i| fences(i, garden))
        .enumerate()
        .map(|(i, f)| (f, i))
        .collect();

    let mut uf = UnionFind::new(all_fences.len());
    for (fence, &index) in all_fences.iter() {
        if let Some(&next) = fence.potentially_next(garden).and_then(|f| all_fences.get(&f)) {
            uf.union(index, next);
        }
    }

    let sides = uf.into_sets().count();

    sides * region.len()
}

fn fences(index: MonadicIndex, garden: &Garden) -> impl Iterator<Item = Fence> + use<'_> {
    let here = garden.to_diadic(index);
    [
        Direction::Left,
        Direction::Right,
        Direction::Up,
        Direction::Down,
    ]
    .into_iter()
    .filter_map(move |side| {
        let needs_fence = garden
            .neighbor(here, side)
            .map_or(true, |neighbor| garden[here] != garden[neighbor]);
        needs_fence.then_some(Fence {
            location: here,
            side,
        })
    })
}

#[derive(Debug)]
struct Garden {
    rows: usize,
    cols: usize,
    plots: Vec<u8>,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct MonadicIndex(usize);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct DiadicIndex {
    row: usize,
    col: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Fence {
    location: DiadicIndex,
    side: Direction,
}


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn counter_clockwise(self) -> Direction {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }
}

impl Fence {
    // next fence, the direction is defined such that
    // the inside is to the left
    fn potentially_next(&self, garden: &Garden) -> Option<Fence> {
        garden
            .neighbor(self.location, self.side.counter_clockwise())
            .map(|l| Fence {
                location: l,
                side: self.side,
            })
    }
}

impl Garden {
    fn new(input: &str) -> Self {
        let (rows, cols) = (
            input.chars().take_while(|&c| c != '\n').count(),
            input.lines().count(),
        );
        let plots = input
            .lines()
            .flat_map(|l| l.as_bytes().to_owned())
            .collect();
        Garden { rows, cols, plots }
    }

    fn iter_regions(&self) -> impl Iterator<Item = HashSet<MonadicIndex>> {
        let mut uf = UnionFind::new(self.plots.len());
        let mut union = |x, y| uf.union(self.to_monadic(x).0, self.to_monadic(y).0);

        for i in 0..self.rows * self.cols {
            let here = self.to_diadic(MonadicIndex(i));
            if let Some(down) = self
                .neighbor(here, Direction::Down)
                .filter(|&down| self[down] == self[here])
            {
                union(here, down);
            }

            if let Some(right) = self
                .neighbor(here, Direction::Right)
                .filter(|&right| self[right] == self[here])
            {
                union(here, right);
            }
        }

        uf.into_sets()
            .map(|s| s.into_iter().map(|i| MonadicIndex(i)).collect())
    }

    fn to_monadic(&self, index: DiadicIndex) -> MonadicIndex {
        if index.row >= self.rows || index.col >= self.cols {
            panic!("index out of range");
        }

        MonadicIndex(index.row * self.cols + index.col)
    }

    fn to_diadic(&self, index: MonadicIndex) -> DiadicIndex {
        DiadicIndex {
            row: index.0 / self.cols,
            col: index.0 % self.cols,
        }
    }

    fn neighbor(&self, location: DiadicIndex, direction: Direction) -> Option<DiadicIndex> {
        match direction {
            Direction::Up => (location.row > 0).then(|| DiadicIndex {
                row: location.row - 1,
                ..location
            }),
            Direction::Down => (location.row < self.rows - 1).then(|| DiadicIndex {
                row: location.row + 1,
                ..location
            }),
            Direction::Left => (location.col > 0).then(|| DiadicIndex {
                col: location.col - 1,
                ..location
            }),
            Direction::Right => (location.col < self.cols - 1).then(|| DiadicIndex {
                col: location.col + 1,
                ..location
            }),
        }
    }
}

impl Index<MonadicIndex> for Garden {
    type Output = u8;
    fn index(&self, index: MonadicIndex) -> &Self::Output {
        &self.plots[index.0]
    }
}

impl Index<DiadicIndex> for Garden {
    type Output = u8;
    fn index(&self, index: DiadicIndex) -> &Self::Output {
        &self[self.to_monadic(index)]
    }
}

// partitions a set into disjoint subsets
// every subset is represented by a node
struct UnionFind {
    // stores sets as trees, every element
    // contains index of parent node.
    // roots contain their own index
    nodes: Vec<Node>,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Node {
    // index of the parent, if this points
    // to itself its a root
    parent: usize,
    // number of descendents,
    // only valid for root nodes
    size: usize,
}

// being generic in the index sucks
// impl<I: Into<usize> + From<usize> + Copy + PartialEq + PartialOrd + AddAssign> UnionFind<I> {
impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            nodes: (0..)
                .take(size)
                .map(|i| Node { parent: i, size: 1 })
                .collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        let node = self.nodes[x];
        if node.parent != x {
            self.nodes[x].parent = self.find(node.parent);
            return self.nodes[x].parent;
        }
        x
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x_idx = self.find(x);
        let root_y_idx = self.find(y);

        if root_x_idx == root_y_idx {
            return;
        }

        // to prevent trees from becoming too deep, make sure to add the smaller tree to the larger
        let [smaller_idx, larger_idx] =
            minmax_by_key(root_x_idx, root_y_idx, |&idx| self.nodes[idx].size);

        self.nodes[smaller_idx].parent = larger_idx;
        self.nodes[larger_idx].size += self.nodes[smaller_idx].size;
    }

    fn into_sets(mut self) -> impl Iterator<Item = HashSet<usize>> {
        let mut groups: HashMap<usize, HashSet<usize>> = HashMap::new();

        for i in 0..self.nodes.len() {
            let root = self.find(i);
            groups.entry(root).or_insert_with(HashSet::new).insert(i);
        }

        groups.into_values()
    }
}

// thanks Claude
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_root() {
        let mut uf = UnionFind::new(3);
        assert_eq!(uf.find(0), 0);
        assert_eq!(uf.find(1), 1);
        assert_eq!(uf.find(2), 2);
    }

    #[test]
    fn test_union_basic() {
        let mut uf = UnionFind::new(4);
        uf.union(0, 1);
        assert_eq!(uf.find(0), uf.find(1));
        assert_ne!(uf.find(0), uf.find(2));
    }

    #[test]
    fn test_union_multiple() {
        let mut uf = UnionFind::new(5);
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(3, 4);

        assert_eq!(uf.find(0), uf.find(1));
        assert_eq!(uf.find(1), uf.find(2));
        assert_eq!(uf.find(3), uf.find(4));
        assert_ne!(uf.find(0), uf.find(3));
    }

    #[test]
    fn test_union_same_element() {
        let mut uf = UnionFind::new(3);
        uf.union(1, 1);
        assert_eq!(uf.find(1), 1);
    }

    #[test]
    fn test_path_compression() {
        let mut uf = UnionFind::new(4);
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(2, 3);

        // First find should trigger path compression
        let root = uf.find(3);
        // All nodes should now point directly to root
        assert_eq!(uf.nodes[0].parent, root);
        assert_eq!(uf.nodes[1].parent, root);
        assert_eq!(uf.nodes[2].parent, root);
        assert_eq!(uf.nodes[3].parent, root);
    }

    #[test]
    fn test_disjoint_sets() {
        let mut uf = UnionFind::new(6);
        uf.union(0, 1);
        uf.union(2, 3);
        uf.union(4, 5);

        // Three separate components
        assert_ne!(uf.find(0), uf.find(2));
        assert_ne!(uf.find(0), uf.find(4));
        assert_ne!(uf.find(2), uf.find(4));
    }

    #[test]
    fn test_union_find_into_sets() {
        let mut uf = UnionFind::new(6);

        // Create some unions: {0,1,2}, {3,4}, {5}
        uf.union(0, 1);
        uf.union(1, 2);
        uf.union(3, 4);

        let sets: Vec<HashSet<usize>> = uf.into_sets().collect();

        // Should have 3 sets
        assert_eq!(sets.len(), 3);

        // Check that we have the expected sets (order doesn't matter)
        let mut found_sets = Vec::new();
        for set in sets {
            found_sets.push(set);
        }

        // Sort by size for consistent testing
        found_sets.sort_by_key(|s| s.len());

        assert_eq!(found_sets[0], HashSet::from([5])); // singleton
        assert_eq!(found_sets[1], HashSet::from([3, 4])); // pair
        assert_eq!(found_sets[2], HashSet::from([0, 1, 2])); // triple
    }
}
