use std::{fs, iter::zip, num::ParseIntError, path::PathBuf, str::FromStr};

use itertools::Itertools;

pub fn day07(mut input_path: PathBuf) {
    input_path.push("07.txt");
    let content = fs::read_to_string(input_path).unwrap();

    let equations: Vec<Equation> = content.lines().map(|l| l.parse().unwrap()).collect();

    println!("{:?}", part1(&equations));
    println!("{:?}", part2(&equations));
}

#[derive(Debug)]
struct Equation {
    result: u64,
    terms: Vec<u64>,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

fn part1(equations: &[Equation]) -> u64 {
    equations
        .iter()
        .filter(|e| can_be_true(e, vec![Operator::Add, Operator::Multiply]))
        .map(|e| e.result)
        .sum()
}

fn part2(equations: &[Equation]) -> u64 {
    equations
        .iter()
        .filter(|e| can_be_true(e, vec![Operator::Add, Operator::Multiply, Operator::Concat]))
        .map(|e| e.result)
        .sum()
}

fn eval(a: u64, op: Operator, b: u64) -> u64 {
    match op {
        Operator::Add => a + b,
        Operator::Multiply => a * b,
        Operator::Concat => {
            let mut o = 1;
            while o <= b {
                o *= 10;
            }
            a * o + b
        }
    }
}

fn is_true(equation: &Equation, operators: &[Operator]) -> bool {
    let [init, tail @ ..] = equation.terms.as_slice() else {
        // we checked during parsing
        panic!("RHS needs at least one term");
    };

    equation.result == zip(operators, tail).fold(*init, |acc, (&op, &t)| eval(acc, op, t))

    // Early stopping the fold when we exceed result is surprisingly not appreciably faster.
    // this try_fold is similar to (sum . filter ((>) result) . (scan foldop))

    // zip(operators, tail)
    //     .try_fold(*init, |acc, (&op, &t)| {
    //         let s = eval(acc, op, t);
    //         if s > equation.result {
    //             return Err(s);
    //         }
    //         Ok(s)
    //     })
    //     .is_ok_and(|lhs| lhs == equation.result)
}

fn can_be_true(equation: &Equation, options: Vec<Operator>) -> bool {
    operator_combinations_for(&equation, options).any(|ops| is_true(&equation, &ops))
}

fn operator_combinations_for(
    equation: &Equation,
    options: Vec<Operator>,
) -> Combinations<Operator> {
    Combinations::new((equation.terms.len() - 1) as u32, options)
}

struct Combinations<T> {
    // TODO: Don't need to own that
    options: Vec<T>,
    length: u32,
    current: usize,
}

impl<T> Combinations<T> {
    fn new(length: u32, options: Vec<T>) -> Self {
        Combinations {
            options,
            length,
            current: 0,
        }
    }
}

impl<T: Copy> Iterator for Combinations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let k = self.options.len();
        self.current += 1;
        if self.current > k.pow(self.length) {
            return None;
        }
        let mut v: Vec<T> = Vec::with_capacity(self.length as usize);
        let n = self.current - 1;
        for i in 0..self.length {
            // part 1 was easy, we could map bits to operation: n >> i & 1
            v.push(self.options[(n / k.pow(i)) % k]);
        }
        Some(v)
    }
}

#[derive(Debug)]
struct ParseEquationError;

// so we can use '?' on Result<_, ParseIntError> in a function returning Result<_, ParseEquationError>
impl From<ParseIntError> for ParseEquationError {
    fn from(_: ParseIntError) -> Self {
        ParseEquationError
    }
}

impl FromStr for Equation {
    type Err = ParseEquationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((lhs, rhs)) = s.split(":").next_tuple() else {
            return Err(ParseEquationError);
        };
        let result = lhs.parse()?;
        let terms = rhs
            .split_whitespace()
            .map(|t| t.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()?;
        match terms.len() {
            0 => Err(ParseEquationError),
            _ => Ok(Equation { result, terms }),
        }
    }
}
