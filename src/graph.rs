use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
    ops::Add,
};

pub fn bfs<F, N, U>(start: U, next: F) -> Bfs<F, N>
where
    N: Eq + Hash + Clone,
    U: IntoIterator<Item = N>,
    F: FnMut(&N) -> U,
{
    let todo: VecDeque<N> = start.into_iter().collect();
    let done = HashSet::from_iter(todo.iter().cloned());
    Bfs {
        fun: next,
        todo,
        done,
    }
}

pub struct Bfs<F, N> {
    fun: F,
    todo: VecDeque<N>,
    done: HashSet<N>,
}

impl<F, N, U> Iterator for Bfs<F, N>
where
    N: Eq + Hash + Clone,
    U: IntoIterator<Item = N>,
    F: FnMut(&N) -> U,
{
    type Item = N;
    fn next(&mut self) -> Option<Self::Item> {
        let next = self.todo.pop_back();
        next.map(|t| {
            for n in (&mut self.fun)(&t) {
                if self.done.insert(n.clone()) {
                    self.todo.push_front(n);
                }
            }
            t
        })
    }
}

/// Enumerate all distinct walks on the graph specified by neighbors, from start to end with length at most max_len.
/// A walk is a sequence of not necessarily distinct edges.
/// Specifying max_len ensures that the algorithm terminates, even if the graph contains a cycle,
/// while not putting constraints on the node type
pub fn all_walks<F, N, U>(
    start: N,
    mut neighbors: F,
    end: N,
    max_len: usize,
) -> impl Iterator<Item = Vec<N>>
where
    N: Clone + Eq + Debug,
    F: FnMut(&N) -> U,
    U: IntoIterator<Item = N>,
{
    if start == end {
        return vec![].into_iter();
    }
    let mut all_walks = vec![];
    let mut queue = VecDeque::from(vec![(start.clone(), vec![start])]);

    while let Some((current, walk)) = queue.pop_front() {
        for neighbor in neighbors(&current) {
            let mut new_walk = walk.clone();
            new_walk.push(neighbor.clone());
            if neighbor == end {
                all_walks.push(new_walk);
            } else if new_walk.len() + 1 <= max_len {
                queue.push_back((neighbor, new_walk));
            }
        }
    }

    all_walks.into_iter()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cost {
    Finite(usize),
    Infinity,
}

impl From<usize> for Cost {
    fn from(value: usize) -> Self {
        Cost::Finite(value)
    }
}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add<usize> for Cost {
    type Output = Self;
    fn add(self, other: usize) -> Self {
        match self {
            Cost::Finite(a) => Cost::Finite(a + other),
            Cost::Infinity => Cost::Infinity,
        }
    }
}

#[derive(Eq, PartialEq, Clone)]
struct BoundaryNode<N> {
    cost: Cost,
    node: N,
}

impl<N: Ord> Ord for BoundaryNode<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl<N: Ord> PartialOrd for BoundaryNode<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Cost::Infinity, Cost::Infinity) => Ordering::Equal, // same as IEEE 754
            (Cost::Infinity, _) => Ordering::Greater,
            (_, Cost::Infinity) => Ordering::Less,
            (Cost::Finite(a), Cost::Finite(b)) => a.cmp(b),
        }
    }
}

pub fn dijkstra<N, F, U, P>(
    start: N,
    mut neighbors: F,
    mut end: P,
) -> Option<impl Iterator<Item = N>>
where
    // I don't like that we need Ord, but as a tiebreaker for the heap it seems necessary
    N: Eq + Hash + Clone + Ord + Debug,
    F: FnMut(&N) -> U,
    U: IntoIterator<Item = (usize, N)>,
    P: FnMut(&N) -> bool,
{
    // Implementation adapted from
    // https://doc.rust-lang.org/std/collections/binary_heap/index.html#examples

    let mut boundary = BinaryHeap::new();
    boundary.push(BoundaryNode {
        cost: 0.into(),
        node: start.clone(),
    });

    let unreachable = || (Cost::Infinity, None);
    let mut path_to: HashMap<N, (Cost, Option<N>)> = HashMap::new();
    path_to.insert(start.clone(), (0.into(), None));

    while let Some(BoundaryNode { cost, node }) = boundary.pop() {
        if end(&node) {
            return Some(
                std::iter::successors(Some(node), |n| path_to.remove(n).and_then(|e| e.1))
                    // collect into a vector so that the (potentially large)
                    // hashmap can be dropped
                    .collect::<Vec<_>>()
                    .into_iter()
                    .rev(),
            );
        }

        if path_to.get(&node).unwrap_or(&unreachable()).0 < cost {
            // we've already found a cheaper way
            continue;
        }

        for (edge_cost, neighbor) in neighbors(&node) {
            let neighbor_cost = cost + edge_cost;
            let shortest_path = path_to.entry(neighbor.clone()).or_insert(unreachable());
            if shortest_path.0 > neighbor_cost {
                shortest_path.0 = neighbor_cost;
                shortest_path.1 = Some(node.clone());
                boundary.push(BoundaryNode {
                    cost: neighbor_cost,
                    node: neighbor,
                })
            }
        }
    }
    None
}

// same as dijkstra, but returns all shortest paths instead of just one
pub fn dijkstra_all<N, F, U>(start: N, mut neighbors: F, end: N) -> Vec<Vec<N>>
where
    // I don't like that we need Ord, but as a tiebreaker for the heap it seems necessary
    N: Eq + Hash + Clone + Ord + Debug,
    F: FnMut(&N) -> U,
    U: IntoIterator<Item = (usize, N)>,
{
    let mut boundary = BinaryHeap::new();
    boundary.push(BoundaryNode {
        cost: 0.into(),
        node: start.clone(),
    });

    let unreachable = || (Cost::Infinity, Vec::new());
    let mut path_to: HashMap<N, (Cost, Vec<N>)> = HashMap::new();
    path_to.insert(start.clone(), (0.into(), Vec::new()));

    // need to keep track - whenever we've seen the end AND we handle a node that's not
    // the end, we stop (all paths we'll discover from then will be strictly worse)
    let mut seen_end = false;
    while let Some(BoundaryNode { cost, node }) = boundary.pop() {
        if node == end {
            seen_end = true;
        }
        if seen_end && node != end {
            return all_walks(end, |n| path_to[n].1.clone(), start, path_to.len())
                .map(|mut w| {
                    w.reverse();
                    w
                })
                .collect();
        }

        if path_to.get(&node).unwrap_or(&unreachable()).0 < cost {
            // we've already found a cheaper way
            continue;
        }

        for (edge_cost, neighbor) in neighbors(&node) {
            let neighbor_cost = cost + edge_cost;
            let shortest_path = path_to.entry(neighbor.clone()).or_insert(unreachable());
            if shortest_path.0 == neighbor_cost {
                // we found another way into neighbor with the same cost as we
                // already hat - add it, but the boundary does not change
                shortest_path.1.push(node.clone());
            } else if shortest_path.0 > neighbor_cost {
                // we found a shorter way into neighbor
                shortest_path.0 = neighbor_cost;
                shortest_path.1 = vec![node.clone()];
                boundary.push(BoundaryNode {
                    cost: neighbor_cost,
                    node: neighbor,
                })
            }
        }
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_small_dag() {
        // Create a small directed acyclic graph:
        // 1 -> [2, 3]
        // 2 -> [4]
        // 3 -> [4]
        // 4 -> []
        let graph: HashMap<i32, Vec<i32>> =
            [(1, vec![2, 3]), (2, vec![4]), (3, vec![4]), (4, vec![])]
                .into_iter()
                .collect();

        let result: Vec<i32> =
            bfs(vec![1], |&node| graph.get(&node).unwrap_or(&vec![]).clone()).collect();

        // BFS should visit nodes in breadth-first order
        // Starting from 1, then 2 and 3 (level 1), then 4 (level 2)
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], 1); // First node visited

        // Node 4 should appear only once despite being reachable from both 2 and 3
        assert_eq!(result.iter().filter(|&&x| x == 4).count(), 1);

        // All nodes should be present
        let mut sorted_result = result.clone();
        sorted_result.sort();
        assert_eq!(sorted_result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_loop() {
        // Create a graph with a cycle:
        // 1 -> [2]
        // 2 -> [3, 1]  // 2 points back to 1, creating a cycle
        // 3 -> [4]
        // 4 -> [2]     // 4 points back to 2, extending the cycle
        let graph: HashMap<i32, Vec<i32>> =
            [(1, vec![2]), (2, vec![3, 1]), (3, vec![4]), (4, vec![2])]
                .into_iter()
                .collect();

        let result: Vec<i32> =
            bfs(vec![1], |&node| graph.get(&node).unwrap_or(&vec![]).clone()).collect();

        // Should visit each node exactly once despite the cycles
        assert_eq!(dbg!(&result).len(), 4);

        // Each node should appear exactly once
        for &expected_node in &[1, 2, 3, 4] {
            assert_eq!(result.iter().filter(|&&x| x == expected_node).count(), 1);
        }

        // First node should be the starting node
        assert_eq!(result[0], 1);
    }

    #[test]
    fn test_single_node() {
        // Test with a single node that has no neighbors
        let result: Vec<i32> = bfs(vec![42], |_| Vec::<i32>::new()).collect();

        assert_eq!(result, vec![42]);

        // Test with a single node that points to itself (self-loop)
        let result_self_loop: Vec<i32> =
            bfs(vec![5], |&node| if node == 5 { vec![5] } else { vec![] }).collect();

        // Should visit the node only once, even with self-loop
        assert_eq!(result_self_loop, vec![5]);
    }

    #[test]
    fn test_simple_walk() {
        // Basic smoke test: simple path from 1 -> 2 -> 3
        let neighbors = |node: &i32| -> Vec<i32> {
            match *node {
                1 => vec![2],
                2 => vec![3],
                _ => vec![],
            }
        };

        let result: Vec<_> = all_walks(1, neighbors, 3, 5).collect();
        assert_eq!(result, vec![vec![1, 2, 3]]);
    }

    #[test]
    fn test_multiple_walks() {
        // Smoke test with multiple paths: 1 -> {2,3} -> 4
        let neighbors = |node: &i32| -> Vec<i32> {
            match *node {
                1 => vec![2, 3],
                2 => vec![4],
                3 => vec![4],
                _ => vec![],
            }
        };

        let mut result: Vec<_> = all_walks(1, neighbors, 4, 5).collect();
        result.sort();
        assert_eq!(result, vec![vec![1, 2, 4], vec![1, 3, 4]]);
    }

    // Helper function to create a simple graph as adjacency list
    fn create_simple_graph() -> HashMap<i32, Vec<(usize, i32)>> {
        let mut graph = HashMap::new();
        graph.insert(1, vec![(1, 2), (4, 3)]);
        graph.insert(2, vec![(2, 4), (1, 5)]);
        graph.insert(3, vec![(3, 4), (2, 6)]);
        graph.insert(4, vec![(1, 5)]);
        graph.insert(5, vec![]);
        graph.insert(6, vec![(1, 5)]);
        graph
    }

    #[test]
    fn test_simple_path() {
        let graph = create_simple_graph();
        let neighbors = |node: &i32| graph.get(node).cloned().unwrap_or_default();

        let result = dijkstra(1, neighbors, |node| *node == 5);
        assert!(result.is_some());

        let path: Vec<i32> = result.unwrap().collect();
        assert_eq!(path, vec![1, 2, 5]);
    }

    #[test]
    fn test_start_equals_end() {
        let graph = create_simple_graph();
        let neighbors = |node: &i32| graph.get(node).cloned().unwrap_or_default();

        let result = dijkstra(1, neighbors, |node| *node == 1);
        assert!(result.is_some());

        let path: Vec<i32> = result.unwrap().collect();
        assert_eq!(path, vec![1]);
    }

    #[test]
    fn test_no_path_exists() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![(1, 2)]);
        graph.insert(2, vec![]);
        graph.insert(3, vec![(1, 4)]);
        graph.insert(4, vec![]);

        let neighbors = |node: &i32| graph.get(node).cloned().unwrap_or_default();

        let result = dijkstra(1, neighbors, |node| *node == 3);
        assert!(result.is_none());
    }

    #[test]
    fn test_single_node_graph() {
        let neighbors = |_node: &i32| vec![];

        let result = dijkstra(42, neighbors, |node| *node == 42);
        assert!(result.is_some());

        let path: Vec<i32> = result.unwrap().collect();
        assert_eq!(path, vec![42]);
    }

    #[test]
    fn test_linear_path() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![(1, 2)]);
        graph.insert(2, vec![(1, 3)]);
        graph.insert(3, vec![(1, 4)]);
        graph.insert(4, vec![]);

        let neighbors = |node: &i32| graph.get(node).cloned().unwrap_or_default();

        let result = dijkstra(1, neighbors, |node| *node == 4);
        assert!(result.is_some());

        let path: Vec<i32> = result.unwrap().collect();
        assert_eq!(path, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_chooses_shortest_path() {
        let mut graph = HashMap::new();
        // Direct path: 1 -> 3 (cost 10)
        // Alternative: 1 -> 2 -> 3 (cost 1 + 1 = 2)
        graph.insert(1, vec![(10, 3), (1, 2)]);
        graph.insert(2, vec![(1, 3)]);
        graph.insert(3, vec![]);

        let neighbors = |node: &i32| graph.get(node).cloned().unwrap_or_default();

        let result = dijkstra(1, neighbors, |node| *node == 3);
        assert!(result.is_some());

        let path: Vec<i32> = result.unwrap().collect();
        assert_eq!(path, vec![1, 2, 3]);
    }

    #[test]
    fn test_with_string_nodes() {
        let mut graph = HashMap::new();
        graph.insert("start".to_string(), vec![(2, "middle".to_string())]);
        graph.insert("middle".to_string(), vec![(3, "end".to_string())]);
        graph.insert("end".to_string(), vec![]);

        let neighbors = |node: &String| graph.get(node).cloned().unwrap_or_default();

        let result = dijkstra("start".to_string(), neighbors, |node| node == "end");
        assert!(result.is_some());

        let path: Vec<String> = result.unwrap().collect();
        assert_eq!(
            path,
            vec!["start".to_string(), "middle".to_string(), "end".to_string()]
        );
    }

    #[test]
    fn test_complex_graph_multiple_paths() {
        let mut graph = HashMap::new();
        graph.insert('A', vec![(4, 'B'), (2, 'C')]);
        graph.insert('B', vec![(1, 'C'), (5, 'D')]);
        graph.insert('C', vec![(8, 'D'), (10, 'E')]);
        graph.insert('D', vec![(2, 'E'), (6, 'F')]);
        graph.insert('E', vec![(3, 'F')]);
        graph.insert('F', vec![]);

        let neighbors = |node: &char| graph.get(node).cloned().unwrap_or_default();

        let result = dijkstra('A', neighbors, |node| *node == 'F');
        assert!(result.is_some());

        let path: Vec<char> = result.unwrap().collect();
        // Should find path A -> C -> D -> E -> F (cost: 2 + 8 + 2 + 3 = 15)
        // Alternative A -> B -> D -> E -> F would be (cost: 4 + 5 + 2 + 3 = 14)
        assert_eq!(path, vec!['A', 'B', 'D', 'E', 'F']);
    }

    #[test]
    fn test_zero_weight_edges() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![(0, 2), (5, 3)]);
        graph.insert(2, vec![(0, 3)]);
        graph.insert(3, vec![]);

        let neighbors = |node: &i32| graph.get(node).cloned().unwrap_or_default();

        let result = dijkstra(1, neighbors, |node| *node == 3);
        assert!(result.is_some());

        let path: Vec<i32> = result.unwrap().collect();
        assert_eq!(path, vec![1, 2, 3]);
    }

    #[test]
    fn test_large_weights() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![(1000, 2)]);
        graph.insert(2, vec![(2000, 3)]);
        graph.insert(3, vec![]);

        let neighbors = |node: &i32| graph.get(node).cloned().unwrap_or_default();

        let result = dijkstra(1, neighbors, |node| *node == 3);
        assert!(result.is_some());

        let path: Vec<i32> = result.unwrap().collect();
        assert_eq!(path, vec![1, 2, 3]);
    }

    #[test]
    fn test_cyclic_graph() {
        let mut graph = HashMap::new();
        graph.insert(1, vec![(1, 2)]);
        graph.insert(2, vec![(1, 3), (2, 1)]); // Back edge to 1
        graph.insert(3, vec![(1, 4)]);
        graph.insert(4, vec![]);

        let neighbors = |node: &i32| graph.get(node).cloned().unwrap_or_default();

        let result = dijkstra(1, neighbors, |node| *node == 4);
        assert!(result.is_some());

        let path: Vec<i32> = result.unwrap().collect();
        assert_eq!(path, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_empty_neighbors_closure() {
        let neighbors = |_node: &i32| vec![];

        let result = dijkstra(1, neighbors, |node| *node == 2);
        assert!(result.is_none());
    }

    #[test]
    fn test_neighbors_with_duplicate_edges() {
        let neighbors = |node: &i32| match node {
            1 => vec![(5, 2), (3, 2)], // Two edges to same node, should pick cheaper
            2 => vec![],
            _ => vec![],
        };

        let result = dijkstra(1, neighbors, |node| *node == 2);
        assert!(result.is_some());

        let path: Vec<i32> = result.unwrap().collect();
        assert_eq!(path, vec![1, 2]);
    }

    #[test]
    fn test_with_custom_struct() {
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        struct Point {
            x: i32,
            y: i32,
        }

        let start = Point { x: 0, y: 0 };
        let middle = Point { x: 1, y: 0 };
        let end = Point { x: 1, y: 1 };

        let neighbors = |point: &Point| match (point.x, point.y) {
            (0, 0) => vec![(1, Point { x: 1, y: 0 })],
            (1, 0) => vec![(1, Point { x: 1, y: 1 })],
            _ => vec![],
        };

        let result = dijkstra(start.clone(), neighbors, |point| {
            point.x == 1 && point.y == 1
        });
        assert!(result.is_some());

        let path: Vec<Point> = result.unwrap().collect();
        assert_eq!(path, vec![start, middle, end]);
    }
}
