use std::{collections::HashSet, hash::Hash};

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
