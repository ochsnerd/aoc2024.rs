use std::collections::HashMap;
use std::fs;
use std::hash::Hash;
use std::path::PathBuf;

pub fn day02(mut input_path: PathBuf) {
    input_path.push("02.txt");
    let content = fs::read_to_string(input_path).unwrap();
    let input = parse_input(&content);

    println!("Part 1 {}", part1(&input));
    println!("Part 2 {}", part2(&input));
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|ss| ss.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn part1(reports: &Vec<Vec<i32>>) -> usize {
    reports.iter().filter(|r| is_safe_part1(r)).count()
}

fn part2(reports: &Vec<Vec<i32>>) -> usize {
    reports
        .iter()
        .filter(|report| is_safe_part2(report.to_vec()))
        .count()
}

fn is_safe_part1(report: &Vec<i32>) -> bool {
    // TODO: Is there a bird here?
    let steps_small_enough = report
        .windows(2)
        .map(|pair| {
            let [a, b] = pair else { unreachable!() };
            i32::abs(a - b)
        })
        .all(|x| x < 4);

    let is_strictly_monotonic = 1
        == unique(
            report
                .windows(2)
                .map(|pair| {
                    let [a, b] = pair else { unreachable!() };
                    i32::signum(a - b)
                })
                .collect(),
        )
        .len();
    steps_small_enough && is_strictly_monotonic
}

fn unique<T>(v: Vec<T>) -> Vec<T>
where
    T: Eq + Hash,
{
    v.into_iter()
        .collect::<std::collections::HashSet<T>>()
        .into_iter()
        .collect()
}

fn is_safe_part2(mut report: Vec<i32>) -> bool {
    if is_safe_part2_(report.clone()){
	return true;
    }
    report.reverse();
    is_safe_part2_(report)
}

fn is_safe_part2_(mut report: Vec<i32>) -> bool {
    let signs = count(
        report
            .windows(2)
            .map(|pair| {
                let [a, b] = pair else { unreachable!() };
                (a - b).signum()
            })
            .collect(),
    );
    let majority_sign = signs
        .iter()
        .max_by_key(|(_, value)| *value)
        .map(|(key, _)| key)
        .unwrap();
    let first_problem_position = problem_position(&report, |a, b| {
        (a - b).abs() > 3 || (a - b).signum() != *majority_sign
    });

    if first_problem_position.is_none() {
        return true;
    }
    report.remove(first_problem_position.unwrap() + 1);
    problem_position(&report, |a, b| {
        (a - b).abs() > 3 || (a - b).signum() != *majority_sign
    })
    .is_none()
}

fn count<T>(v: Vec<T>) -> HashMap<T, usize>
where
    T: Eq + Hash,
{
    let mut counter = HashMap::new();

    for element in v {
        *counter.entry(element).or_default() += 1;
    }
    counter
}

fn problem_position<P>(report: &Vec<i32>, mut condition: P) -> Option<usize>
where
    P: FnMut(&i32, &i32) -> bool,
{
    report.windows(2).position(|pair| {
        let [a, b] = pair else { unreachable!() };
        condition(a, b)
    })
}
