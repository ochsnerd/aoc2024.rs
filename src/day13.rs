use std::{iter::from_fn, num::ParseIntError, str::FromStr};
use thiserror::Error;

type StoreInt = u64;
type CalcInt = i128;

const PART2_SHIFT: StoreInt = 10000000000000;

const_assert!(u64::MAX > 2 * PART2_SHIFT);
const_assert!(i128::MAX > 4 * PART2_SHIFT as CalcInt * PART2_SHIFT as CalcInt);

pub fn solve(input: &str) -> (usize, usize) {
    let machines = Machine::parse_all(input)
        .filter_map(|m| m.ok())
        .collect::<Vec<_>>();

    (part1(&machines), part2(&machines))
}

fn part1(machines: &[Machine]) -> usize {
    machines
        .iter()
        .filter_map(|m| m.solve())
        .map(|s| s.cost())
        .sum()
}

fn part2(machines: &[Machine]) -> usize {
    machines
        .iter()
        .map(|m| Machine {
            prize: m.prize.map(|x| x + PART2_SHIFT),
            a_action: m.a_action,
            b_action: m.b_action,
        })
        .filter_map(|m| m.solve())
        .map(|s| s.cost())
        .sum()
}

#[derive(Debug, PartialEq, Eq)]
struct Machine {
    prize: [StoreInt; 2],
    a_action: [StoreInt; 2],
    b_action: [StoreInt; 2],
}

#[derive(Error, Debug)]
enum MachineParseError {
    #[error("Wrong Format")]
    WrongNumberFormat(#[from] ParseIntError),
    #[error("Wrong Format")]
    WrongFormat,
    #[error("Not enough Data")]
    NotEnoughData,
}

impl Machine {
    fn solve(&self) -> Option<Solution> {
        // prize = a_presses * a_action + b_presses * b_action
        // is a 2x2 LSE, use Cramer's rule to solve
        let [p1, p2] = self.prize.map(|x| x as CalcInt);
        let [a1, a2] = self.a_action.map(|x| x as CalcInt);
        let [b1, b2] = self.b_action.map(|x| x as CalcInt);

        let det = a1 * b2 - a2 * b1;

        if det == 0 {
            return None;
        }

        let (a_presses, a_rem) = divmod(b2 * p1 - b1 * p2, det);
        let (b_presses, b_rem) = divmod(a1 * p2 - a2 * p1, det);

        if a_presses < 0 || a_rem != 0 || b_presses < 0 || b_rem != 0 {
            return None;
        }

        return StoreInt::try_from(a_presses)
            .ok()
            .zip_with(StoreInt::try_from(b_presses).ok(), Solution::new);

        fn divmod(x: CalcInt, y: CalcInt) -> (CalcInt, CalcInt) {
            (x / y, x % y)
        }
    }
}

impl FromStr for Machine {
    type Err = MachineParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_button(line: &str, name: &str) -> Result<(StoreInt, StoreInt), MachineParseError> {
            line.strip_prefix(&format!("Button {}: X+", name))
                .and_then(|rest| rest.split_once(", Y+"))
                .ok_or(MachineParseError::WrongFormat) // Would want WrongFormat, but then the typechecker gets confused
                .and_then(|(x, y)| Ok((x.parse()?, y.parse()?)))
        }

        fn parse_prize(line: &str) -> Result<(StoreInt, StoreInt), MachineParseError> {
            line.strip_prefix("Prize: X=")
                .and_then(|rest| rest.split_once(", Y="))
                .ok_or(MachineParseError::WrongFormat)
                .and_then(|(x, y)| Ok((x.parse()?, y.parse()?)))
        }

        let mut lines = s.lines();
        let a_line = lines.next().ok_or(MachineParseError::NotEnoughData)?;
        let b_line = lines.next().ok_or(MachineParseError::NotEnoughData)?;
        let p_line = lines.next().ok_or(MachineParseError::NotEnoughData)?;

        let (a_x, a_y) = parse_button(a_line, "A")?;
        let (b_x, b_y) = parse_button(b_line, "B")?;
        let (p_x, p_y) = parse_prize(p_line)?;

        Ok(Machine {
            prize: [p_x, p_y],
            a_action: [a_x, a_y],
            b_action: [b_x, b_y],
        })
    }
}

trait ParseIncrementally: FromStr {
    const NUM_LINES: usize;

    fn incremental_parse(s: &str) -> (Result<Self, Self::Err>, &str) {
        fn split_n_newlines(n: usize, s: &str) -> (&str, &str) {
            if n == 0 {
                return ("", &s);
            }
            let newlines: Vec<_> = s.match_indices('\n').take(n).collect();
            if newlines.len() == n {
                let split_pos = newlines[n - 1].0;
                (&s[..split_pos], &s[split_pos + 1..])
            } else {
                (&s, "")
            }
        }

        let (start, rest) = split_n_newlines(Self::NUM_LINES, s);
        (start.parse(), rest)
    }

    fn parse_all(s: &str) -> impl Iterator<Item = Result<Self, Self::Err>> {
        let mut rem = s;
        from_fn(move || {
            if rem.is_empty() {
                return None;
            }
            let (result, rest) = Self::incremental_parse(rem);
            rem = rest;
            Some(result)
        })
    }
}

impl ParseIncrementally for Machine {
    const NUM_LINES: usize = 4;
}

#[derive(Debug, PartialEq, Eq)]
struct Solution {
    a_presses: StoreInt,
    b_presses: StoreInt,
}

impl Solution {
    fn new(a_presses: StoreInt, b_presses: StoreInt) -> Self {
        Self {
            a_presses,
            b_presses,
        }
    }

    fn cost(&self) -> usize {
        self.a_presses as usize * 3 + self.b_presses as usize * 1
    }
}
