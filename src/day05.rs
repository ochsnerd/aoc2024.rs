use std::collections::{HashMap, HashSet};
use std::fs;

use itertools::Itertools;

pub fn day05(input_path: String) {
    let input = fs::read_to_string(input_path).unwrap();

    let (rules, pages_list) = parse_input(&input);

    println!("{:?}", part1(&pages_list, &rules));
    println!("{:?}", part2(&pages_list, &rules));
}

type Graph = HashMap<u32, HashSet<u32>>;
type Pages = Vec<u32>;

fn parse_input(input: &str) -> (Graph, Vec<Pages>) {
    let mut rules = HashMap::new();
    for l in input.lines().take_while(|l| !l.is_empty()) {
        let Some((Ok(before), Ok(after))) = l.split("|").map(|n| n.parse::<u32>()).next_tuple()
        else {
            panic!("The input does not look like expected");
        };

        rules.entry(before).or_insert(HashSet::new()).insert(after);
    }

    let pages_list = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .skip(1) // empty delineating line
        .map(|l| l.split(",").map(|n| n.parse::<u32>().unwrap()).collect())
        .collect();

    (rules, pages_list)
}

fn part1(pages_list: &Vec<Pages>, rules: &Graph) -> u32 {
    pages_list
        .iter()
        .filter(|pages| is_correctly_ordered(pages, rules))
        .map(|pages| middle_of(pages))
        .sum()
}

fn is_correctly_ordered(pages: &[u32], rules: &Graph) -> bool {
    for pages in inits(pages) {
        if let [init @ .., last] = pages {
            if rules
                .get(last)
                .is_some_and(|befores| init.iter().any(|p| befores.contains(p)))
            {
                return false;
            }
        }
    }
    true
}

fn middle_of<T>(values: &[T]) -> &T {
    &values[values.len() / 2]
}

fn part2(pages_list: &Vec<Pages>, rules: &Graph) -> u32 {
    // note that we cannot just topological_sort(rules) because they contain cycles
    pages_list
        .iter()
        .filter(|pages| !is_correctly_ordered(pages, rules))
        .map(|pages| order_correctly(pages, rules))
        .map(|pages| *middle_of(&pages))
        .sum()
}

fn order_correctly(pages: &[u32], rules: &Graph) -> Vec<u32> {
    let mut dependencies = Graph::new();
    for (before, &start, after) in split_everywhere(pages) {
        for &end in before.iter().chain(after) {
            // if path_between(start, end, rules) {
            if rules.get(&start).is_some_and(|ends| ends.contains(&end)) {
                dependencies
                    .entry(start)
                    .or_insert(HashSet::new())
                    .insert(end);
            }
        }
    }
    topological_sort(&dependencies)
}

fn topological_sort(graph: &Graph) -> Vec<u32> {
    // See DFS algorithm in
    // https://en.wikipedia.org/wiki/Topological_sorting

    // rust functions are pure (i.e. have no closure)
    // It would be nice to capture finished, doing in a (rust-)closure,
    // but then there's no way to recursively call itself
    fn visit(
        node: u32,
        graph: &Graph,
        finished: &mut HashSet<u32>,
        doing: &mut HashSet<u32>,
        sorted_reversed: &mut Vec<u32>,
    ) {
        if finished.contains(&node) {
            return;
        }
        if doing.contains(&node) {
            panic!("We're in a loop!");
        }
        doing.insert(node);
        for next in graph.get(&node).unwrap_or(&HashSet::new()) {
            visit(*next, graph, finished, doing, sorted_reversed);
        }
        finished.insert(node);
        sorted_reversed.push(node);
    }

    let nodes = graph.keys().cloned().collect::<HashSet<u32>>();
    let mut finished = HashSet::new();
    let mut doing: HashSet<u32> = HashSet::new();
    let mut sorted: Vec<u32> = Vec::new(); // reversed

    // note the copied
    while let Some(next) = nodes.difference(&finished).copied().next() {
        visit(next, graph, &mut finished, &mut doing, &mut sorted);
    }
    sorted.reverse();
    sorted
}

fn inits<T>(v: &[T]) -> Vec<&[T]> {
    // inits([1, 2, 3]) -> [[], [1], [1, 2], [1, 2, 3]]
    // https://hackage.haskell.org/package/base-4.21.0.0/docs/Data-List.html#v:inits
    let mut result = Vec::with_capacity(v.len() + 1);
    for i in 0..=v.len() {
        result.push(&v[..i]);
    }
    result
}

fn split_everywhere<T>(v: &[T]) -> Vec<(&[T], &T, &[T])> {
    // split_everywhere([1, 2, 3]) -> [([], 1, [2, 3]), ([1], 2, [3]), ([1, 2], 3, [])]
    // https://hackage.haskell.org/package/utility-ht-0.0.17.2/docs/Data-List-HT.html#v:splitEverywhere
    let mut result = Vec::with_capacity(v.len());
    for i in 0..v.len() {
        result.push((&v[..i], &v[i], &v[i + 1..]));
    }
    result
}
