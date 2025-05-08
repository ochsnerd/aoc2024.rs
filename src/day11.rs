use std::{collections::HashMap, fs, hash::Hash, ops::AddAssign};

pub fn day11(input_path: String) {
    let content = fs::read_to_string(input_path).unwrap();

    let stones: Vec<_> = content.split_whitespace().map(Stone::parse).collect();

    println!("{:?}", part1(stones.iter().cloned().collect()));
    println!("{:?}", part2(stones));
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

fn part2(stones: Vec<Stone>) -> usize {
    let mut stone_map: HashMap<Stone, usize> =
        accumulate_into_hashmap(stones.into_iter().map(|s| (s, 1)));

    for _ in 0..75 {
        stone_map = accumulate_into_hashmap(
            stone_map
                .into_iter()
                .flat_map(|(stone, count)| blink_at(stone).into_iter().map(move |s| (s, count))),
        );
    }
    stone_map.iter().map(|(_, count)| count).sum()
}

fn accumulate_into_hashmap<T, N, I>(values: I) -> HashMap<T, N>
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

fn blink_at(stone: Stone) -> Vec<Stone> {
    match stone {
        zero if zero.0.is_empty() => vec![Stone::from([1])],
        even if even.0.len() % 2 == 0 => Vec::from(even.split()),
        other => vec![other.muled_by(2024)],
    }
}

impl Stone {
    // should actually be proper parse
    fn parse(string: &str) -> Stone {
        Stone::new(
            string
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .rev()
                .collect(),
        )
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_muled_by_zero() {
        let stone = Stone::parse("123");
        let result = stone.muled_by(0);
        assert_eq!(result.0, vec![0, 0, 0]); // 123 * 0 = 0
    }

    #[test]
    fn test_muled_by_one() {
        let stone = Stone::parse("456");
        let result = stone.muled_by(1);
        assert_eq!(result.0, vec![6, 5, 4]); // 456 * 1 = 456
    }

    #[test]
    fn test_muled_by_large_number() {
        let stone = Stone::parse("999");
        let result = stone.muled_by(999);
        // 999 * 999 = 998001
        assert_eq!(result.0, vec![1, 0, 0, 8, 9, 9]);
    }
}
