use std::{collections::HashMap, fs, hash::Hash, num::ParseIntError, ops::AddAssign, str::FromStr};

pub fn day11(input_path: String) {
    let content = fs::read_to_string(input_path).unwrap();

    let stones: Vec<_> = content
        .split_whitespace()
        .map(|l| l.parse().unwrap())
        .collect();

    println!("{:?}", part1(stones.iter().cloned().collect()));
    println!("{:?}", part2(&stones));

    let stones: Vec<_> = content
        .split_whitespace()
        .map(|l| l.parse().unwrap()) // this deduces Stone2 because of the signature of part12
        .collect();

    println!("{:?}", part12(&stones));
    println!("{:?}", part22(&stones));
}

// Most significant digit is last,
// leading zeros are trimmed  <- interesting: how do we enforce that?
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Stone(Vec<u8>);

fn part1(stones: Vec<Stone>) -> usize {
    std::iter::successors(Some(stones), |stones| {
        Some(stones.iter().flat_map(|s| blink_at(s.clone())).collect())
    })
    .take(26)
    .last()
    .unwrap()
    .len()
}

fn part2(stones: &[Stone]) -> usize {
    // order does not matter, so we can compress the stones into a hashmap, and do every operation just once
    let mut stone_map: HashMap<Stone, usize> = accumulate(stones.iter().map(|s| (s.clone(), 1)));

    for _ in 0..75 {
        stone_map = accumulate(
            stone_map
                .into_iter()
                .flat_map(|(stone, count)| blink_at(stone).into_iter().map(move |s| (s, count))),
        );
    }
    stone_map.iter().map(|(_, count)| count).sum()
}

fn blink_at(stone: Stone) -> Vec<Stone> {
    match stone {
        zero if zero.0.is_empty() => vec![Stone::from([1])],
        even if even.0.len() % 2 == 0 => Vec::from(even.split()),
        other => vec![other.muled_by(2024)],
    }
}

impl Stone {
    fn new(digits: Vec<u8>) -> Stone {
        Stone(digits).trimmed()
    }

    fn from<const N: usize>(arr: [u8; N]) -> Self {
        Stone::new(Vec::from(arr))
    }

    fn split(mut self) -> [Stone; 2] {
        let second = self.0.split_off(self.0.len() / 2);
        [Stone::new(second), self.trimmed()]
    }

    fn trimmed(mut self) -> Self {
        while self.0.last() == Some(&0) {
            self.0.pop();
        }
        self
    }

    fn muled_by(mut self, n: usize) -> Self {
        let mut carry = 0usize;
        for d in self.0.iter_mut() {
            let value = (*d as usize) * n + carry;
            *d = (value % 10) as u8;
            carry = value / 10;
        }

        while carry != 0 {
            self.0.push((carry % 10) as u8);
            carry /= 10;
        }
        self
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ParseStoneError;

impl FromStr for Stone {
    type Err = ParseStoneError;

    // this is just for illustrating
    // fn from_str(s: &str) -> Result<Self, Self::Err> {
    //     let digits: Result<Vec<u8>, ParseStoneError> = s
    //         .chars()
    //         .map(|c| c.to_digit(10).map(|d| d as u8).ok_or(ParseStoneError))
    //         .rev()
    //         // this collect flips and creates a vector,
    //         // i.e. it does
    //         // Iter<Result<u8, Error>> -> Result<Vec<u8>, Error>
    //         // i.e. it does "early return with Result"
    //         .collect();
    //     Ok(Stone::new(digits?))
    // }

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .map(|c| c.to_digit(10).map(|d| d as u8).ok_or(ParseStoneError))
            .rev()
            .collect::<Result<_, _>>()
            .map(Stone::new)
    }
}

fn accumulate<T, N, I>(values: I) -> HashMap<T, N>
where
    T: Eq + Hash,
    N: Default + AddAssign,
    I: IntoIterator<Item = (T, N)>,
{
    values
        .into_iter()
        .fold(HashMap::new(), |mut acc, (key, value)| {
            *acc.entry(key).or_default() += value;
            acc
        })
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Stone2 {
    number: usize,
    digits: u32,
}

fn digits(number: usize) -> u32 {
    if number == 0 {
        return 1;
    }
    let mut temp = number;
    let mut digit_count = 0;
    while temp > 0 {
        temp /= 10;
        digit_count += 1;
    }
    digit_count
}

impl Stone2 {
    fn new(number: usize) -> Self {
        Stone2 {
            number,
            digits: digits(number),
        }
    }

    fn split(self) -> [Self; 2] {
        // calling this with odd number of digits is ub (I'm lazy)
        let divisor = 10usize.pow(self.digits / 2);
        let left = Stone2 {
            number: self.number / divisor,
            digits: self.digits / 2,
        };
        // right could have leading zeros, recompute digits
        let right = Stone2::new(self.number % divisor);
        [left, right]
    }

    fn muled_by(self, n: usize) -> Self {
        Stone2::new(self.number * n)
    }
}

// here, returning impl Iterator<Item = Stone2>
// does not work, because all match arms would need to have the same concrete type
// (which they cannot have, except when we'd implement a bespoke OneOrTwo-struct)
fn blink_at2(stone: Stone2) -> Vec<Stone2> {
    match stone {
        Stone2 {
            number: 0,
            digits: 1,
        } => vec![Stone2 {
            number: 1,
            digits: 1,
        }],
        even if even.digits % 2 == 0 => Vec::from(even.split()),
        other => vec![other.muled_by(2024)],
    }
}

fn do_rounds(stones: &[Stone2], rounds: usize) -> usize {
    let mut stone_map = accumulate(stones.iter().map(|s| (s.clone(), 1)));

    for _ in 0..rounds {
        stone_map = accumulate(
            stone_map
                .into_iter()
                .flat_map(|(stone, count)| blink_at2(stone).into_iter().map(move |s| (s, count))),
        );
    }
    stone_map.iter().map(|(_, count)| count).sum()
}

fn part12(stones: &[Stone2]) -> usize {
    do_rounds(stones, 25)
}

fn part22(stones: &[Stone2]) -> usize {
    do_rounds(stones, 75)
}

impl FromStr for Stone2 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(|ds| Stone2::new(ds))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_muled_by_zero() {
        let stone: Stone = "123".parse().unwrap();
        let result = stone.muled_by(0);
        assert_eq!(result.0, vec![0, 0, 0]); // 123 * 0 = 0
    }

    #[test]
    fn test_muled_by_one() {
        let stone: Stone = "456".parse().unwrap();
        let result = stone.muled_by(1);
        assert_eq!(result.0, vec![6, 5, 4]); // 456 * 1 = 456
    }

    #[test]
    fn test_muled_by_large_number() {
        let stone: Stone = "999".parse().unwrap();
        let result = stone.muled_by(999);
        // 999 * 999 = 998001
        assert_eq!(result.0, vec![1, 0, 0, 8, 9, 9]);
    }

    #[test]
    fn test_muled_by_zero2() {
        let stone: Stone2 = "123".parse().unwrap();
        let result = stone.muled_by(0);
        assert_eq!(
            result,
            Stone2 {
                number: 0,
                digits: 1
            }
        ); // 123 * 0 = 0
    }

    #[test]
    fn test_muled_by_one2() {
        let stone: Stone2 = "456".parse().unwrap();
        let result = stone.muled_by(1);
        assert_eq!(
            result,
            Stone2 {
                number: 456,
                digits: 3
            }
        ); // 456 * 1 = 456
    }

    #[test]
    fn test_muled_by_large_number2() {
        let stone: Stone2 = "999".parse().unwrap();
        let result = stone.muled_by(999);
        assert_eq!(
            result,
            Stone2 {
                number: 998001,
                digits: 6
            }
        ); // 999 * 999 = 998001
    }

    #[test]
    fn test_split() {
        let stone: Stone2 = "1001".parse().unwrap();
        let result = stone.split();
        assert_eq!(result, [Stone2::new(10), Stone2::new(1)]);
    }
}
