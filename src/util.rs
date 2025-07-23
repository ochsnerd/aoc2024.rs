use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
};

// upside: can be chained nicely, is lazy
// downside: need clonable, up to 2x memory requirements
pub struct Uniques<I, T>
where
    T: Eq + Hash + Clone,
    I: Iterator<Item = T>,
{
    input: I,
    seen: HashSet<T>,
}

impl<I, T> Uniques<I, T>
where
    T: Eq + Hash + Clone,
    I: Iterator<Item = T>,
{
    fn new(input: I) -> Self {
        Self {
            input,
            seen: HashSet::new(),
        }
    }
}

impl<I, T> Iterator for Uniques<I, T>
where
    T: Eq + Hash + Clone,
    I: Iterator<Item = T>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(item) = self.input.next() {
            if self.seen.insert(item.clone()) {
                return Some(item);
            }
        }
        None
    }
}

// Extension trait to make it convenient to use
pub trait IteratorExt: Iterator {
    fn uniques(self) -> Uniques<Self, Self::Item>
    where
        Self: Sized,
        Self::Item: Eq + Hash + Clone,
    {
        Uniques::new(self)
    }
}

impl<I: Iterator> IteratorExt for I {}

pub fn bfs<F, T, U>(start: U, next: F) -> Bfs<F, T>
where
    T: Eq + Hash + Clone,
    U: IntoIterator<Item = T>,
    F: FnMut(&T) -> U,
{
    Bfs::new(next, start.into_iter().collect())
}

pub struct Bfs<F, T> {
    fun: F,
    todo: VecDeque<T>,
    done: HashSet<T>,
}

impl<F, T> Bfs<F, T>
where
    T: Eq + Hash + Clone,
{
    pub fn new(fun: F, todo: VecDeque<T>) -> Bfs<F, T> {
        let done = HashSet::from_iter(todo.iter().cloned());
        Self { fun, todo, done }
    }
}

impl<F, T, U> Iterator for Bfs<F, T>
where
    T: Eq + Hash + Clone,
    U: IntoIterator<Item = T>,
    F: FnMut(&T) -> U,
{
    type Item = T;
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_uniques() {
        let input = vec![1, 2, 3, 2, 4, 1, 5];
        let result: Vec<i32> = input.into_iter().uniques().collect();
        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_uniques_strings() {
        let input = vec!["a", "b", "c", "b", "a", "d"];
        let result: Vec<&str> = input.into_iter().uniques().collect();
        assert_eq!(result, vec!["a", "b", "c", "d"]);
    }

    #[test]
    fn test_empty() {
        let input: Vec<i32> = vec![];
        let result: Vec<i32> = input.into_iter().uniques().collect();
        assert_eq!(result, vec![]);
    }

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
}
