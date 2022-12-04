use advent_of_code_2022::stream_items_from_file;
use std::num::ParseIntError;
use std::ops::Range;
use std::{path::Path, str::FromStr};
use thiserror::Error;

use anyhow::Result;

const INPUT: &str = "input/day04.txt";

struct CleaningRangePair(Range<usize>, Range<usize>);

#[derive(Error, Clone, Debug)]
enum CleaningRangeParsingError {
    #[error("Invalid pair '{0}'")]
    InvalidPair(String),
    #[error("Invalid range '{0}'")]
    InvalidRange(String),
    #[error("Invalid range limit")]
    InvalidRangeLimit(#[from] ParseIntError),
}

impl CleaningRangePair {
    fn parse_range(s: &str) -> Result<Range<usize>, CleaningRangeParsingError> {
        let (from, to) = s
            .split_once('-')
            .ok_or_else(|| CleaningRangeParsingError::InvalidRange(s.to_string()))?;

        Ok(Range {
            start: from.parse()?,
            end: to.parse::<usize>()? + 1,
        })
    }
}

impl FromStr for CleaningRangePair {
    type Err = CleaningRangeParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (val_a, val_b) = s
            .split_once(',')
            .ok_or_else(|| CleaningRangeParsingError::InvalidPair(s.to_string()))?;

        Ok(Self(Self::parse_range(val_a)?, Self::parse_range(val_b)?))
    }
}

trait RangeSubset {
    fn fully_contains(&self, other: &Self) -> bool;
    fn overlaps_start(&self, other: &Self) -> bool;
}

impl RangeSubset for Range<usize> {
    fn fully_contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps_start(&self, other: &Self) -> bool {
        self.start <= other.start && self.end > other.start
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    Ok(stream_items_from_file::<P, CleaningRangePair>(input)?
        .map(|p| p.expect("Invalid range descriptor"))
        .filter(|p| p.0.fully_contains(&p.1) || p.1.fully_contains(&p.0))
        .count())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    Ok(stream_items_from_file::<P, CleaningRangePair>(input)?
        .map(|p| p.expect("Invalid range descriptor"))
        .filter(|p| p.0.overlaps_start(&p.1) || p.1.overlaps_start(&p.0))
        .count())
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
    fn test_d04_examples() {
        let (dir, file) = create_example_file(
            indoc! {"
                2-4,6-8
                2-3,4-5
                5-7,7-9
                2-8,3-7
                6-6,4-6
                2-6,4-8
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 2);
        assert_eq!(part2(&file).unwrap(), 4);
        drop(dir);
    }
}
