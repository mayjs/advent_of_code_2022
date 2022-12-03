use advent_of_code_2022::stream_items_from_file;
use itertools::Itertools;
use std::{collections::HashSet, path::Path, str::FromStr};
use thiserror::Error;

use anyhow::Result;

const INPUT: &str = "input/day03.txt";

// TODO: A more efficient representation would be to convert items to their priority immediately
// and then use a 64 bit BitSet to represent the Rucksack pockets

#[derive(Debug, Error)]
enum RucksackParsingError {
    #[error("Invalid item '{0}'")]
    InvalidItem(char),
    #[error("Invalid Rucksack legnth '{0}'")]
    InvalidLength(usize),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Item(char);

impl Item {
    fn new(representation: char) -> Result<Item, RucksackParsingError> {
        if representation.is_ascii_alphabetic() {
            Ok(Item(representation))
        } else {
            Err(RucksackParsingError::InvalidItem(representation))
        }
    }

    fn priority(&self) -> usize {
        if self.0.is_ascii_lowercase() {
            self.0 as usize - 'a' as usize + 1
        } else if self.0.is_ascii_uppercase() {
            self.0 as usize - 'A' as usize + 27
        } else {
            panic!("Invalid Item")
        }
    }
}

struct Rucksack(HashSet<Item>, HashSet<Item>);

impl FromStr for Rucksack {
    type Err = RucksackParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() % 2 == 0 {
            let first_pocket = s[0..s.len() / 2]
                .chars()
                .map(|c| Item::new(c))
                .collect::<Result<HashSet<_>, _>>()?;
            let second_pocket = s[s.len() / 2..]
                .chars()
                .map(|c| Item::new(c))
                .collect::<Result<HashSet<_>, _>>()?;

            Ok(Rucksack(first_pocket, second_pocket))
        } else {
            Err(RucksackParsingError::InvalidLength(s.len()))
        }
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    Ok(stream_items_from_file::<P, Rucksack>(input)?
        .map(|maybe_rucksack| maybe_rucksack.expect("Invalid Rucksack descriptor"))
        .map(|r| r.0.intersection(&r.1).map(|i| i.priority()).sum::<usize>())
        .sum())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    Ok(stream_items_from_file::<P, Rucksack>(input)?
        .map(|maybe_rucksack| maybe_rucksack.expect("Invalid Rucksack descriptor"))
        .tuples()
        .map(|(r1, r2, r3)| {
            (&(&r1.0 | &r1.1) & &(&r2.0 | &r2.1))
                .intersection(&(&r3.0 | &r3.1))
                .next()
                .unwrap()
                .priority()
        })
        .sum())
}

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2: {}", part2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::test_helpers::create_example_file;
    use indoc::indoc;

    #[test]
    fn test_d02_examples() {
        let (dir, file) = create_example_file(
            indoc! {"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 157);
        assert_eq!(part2(&file).unwrap(), 70);
        drop(dir);
    }
}
