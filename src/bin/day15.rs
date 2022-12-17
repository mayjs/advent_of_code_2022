use advent_of_code_2022::stream_items_from_file;
use anyhow::anyhow;
use anyhow::Result;
use itertools::{chain, Itertools};
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashSet, path::Path, str::FromStr};
use thiserror::Error;

const INPUT: &str = "input/day15.txt";

type Coordinate = (isize, isize);

struct Sensor {
    location: Coordinate,
    closest_beacon: Coordinate,
}

#[derive(Error, Debug)]
enum SensorParseError {
    #[error("Invalid descriptor")]
    InvalidDescriptor,
}

lazy_static! {
    static ref NUMBER_REGEX: Regex = Regex::new(r"-?\d+").unwrap();
}

impl FromStr for Sensor {
    type Err = SensorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (location, closest_beacon) = NUMBER_REGEX
            .find_iter(s)
            .map(|found| found.as_str().parse::<isize>().unwrap())
            .tuples()
            .tuples()
            .next()
            .ok_or(SensorParseError::InvalidDescriptor)?;
        Ok(Sensor {
            location,
            closest_beacon,
        })
    }
}

fn manhattan_distance(a: &Coordinate, b: &Coordinate) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

impl Sensor {
    fn covers(&self, coord: &Coordinate) -> bool {
        let range = manhattan_distance(&self.location, &self.closest_beacon);
        manhattan_distance(&self.location, coord) <= range
    }

    fn get_min_x(&self) -> isize {
        let range = manhattan_distance(&self.location, &self.closest_beacon);
        self.location.0 - (range as isize)
    }

    fn get_max_x(&self) -> isize {
        let range = manhattan_distance(&self.location, &self.closest_beacon);
        self.location.0 + (range as isize)
    }

    fn range(&self) -> usize {
        manhattan_distance(&self.location, &self.closest_beacon)
    }

    /// Get a ring of candidates around the range of this sensor.
    fn get_uncovered_candidates(&self) -> impl Iterator<Item = Coordinate> + '_ {
        chain![
            (0..self.range() as isize + 1).map(|i| {
                (
                    self.location.0 + self.range() as isize + 1 - i,
                    self.location.1 + i,
                )
            }),
            (0..self.range() as isize).map(|i| {
                (
                    self.location.0 + i,
                    self.location.1 + self.range() as isize + 1 - i,
                )
            }),
            (0..self.range() as isize).map(|i| {
                (
                    self.location.0 - self.range() as isize - 1 + i,
                    self.location.1 + i,
                )
            }),
            (0..self.range() as isize).map(|i| {
                (
                    self.location.0 + i,
                    self.location.1 - self.range() as isize - 1 + i,
                )
            }),
        ]
    }
}

fn part1<P: AsRef<Path>>(input: P, row: isize) -> Result<usize> {
    // This is a inefficient solution, lots of hashmap lookups and stuff...
    let sensors = stream_items_from_file::<P, Sensor>(input)?.collect::<Result<Vec<_>, _>>()?;
    let min_x = sensors.iter().map(|s| s.get_min_x()).min().unwrap();
    let max_x = sensors.iter().map(|s| s.get_max_x()).max().unwrap();
    let beacons = sensors
        .iter()
        .map(|s| s.closest_beacon)
        .collect::<HashSet<_>>();

    Ok((min_x..=max_x)
        .filter(|x| !beacons.contains(&(*x, row)) && sensors.iter().any(|s| s.covers(&(*x, row))))
        .count())
}

fn part2<P: AsRef<Path>>(input: P, xlim: isize, ylim: isize) -> Result<usize> {
    let sensors = stream_items_from_file::<P, Sensor>(input)?.collect::<Result<Vec<_>, _>>()?;
    for sensor in &sensors {
        if let Some(coordinates) = sensor
            .get_uncovered_candidates()
            .filter(|cand| {
                cand.0 >= 0
                    && cand.1 >= 0
                    && cand.0 <= xlim
                    && cand.1 <= ylim
                    && !sensors.iter().any(|s| s.covers(&cand))
            })
            .next()
        {
            return Ok((coordinates.0 as usize * 4000000) + coordinates.1 as usize);
        }
    }

    Err(anyhow!("No solution!"))
}

fn main() -> Result<()> {
    const P1ROW: isize = 2000000;
    println!("Answer for part 1: {}", part1(INPUT, P1ROW)?);
    const P2LIMIT: isize = 4000000;
    println!("Answer for part 2: {}", part2(INPUT, P2LIMIT, P2LIMIT)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::test_helpers::create_example_file;
    use indoc::indoc;

    #[test]
    fn test_d14_examples() {
        let (dir, file) = create_example_file(
            indoc! {"
                Sensor at x=2, y=18: closest beacon is at x=-2, y=15
                Sensor at x=9, y=16: closest beacon is at x=10, y=16
                Sensor at x=13, y=2: closest beacon is at x=15, y=3
                Sensor at x=12, y=14: closest beacon is at x=10, y=16
                Sensor at x=10, y=20: closest beacon is at x=10, y=16
                Sensor at x=14, y=17: closest beacon is at x=10, y=16
                Sensor at x=8, y=7: closest beacon is at x=2, y=10
                Sensor at x=2, y=0: closest beacon is at x=2, y=10
                Sensor at x=0, y=11: closest beacon is at x=2, y=10
                Sensor at x=20, y=14: closest beacon is at x=25, y=17
                Sensor at x=17, y=20: closest beacon is at x=21, y=22
                Sensor at x=16, y=7: closest beacon is at x=15, y=3
                Sensor at x=14, y=3: closest beacon is at x=15, y=3
                Sensor at x=20, y=1: closest beacon is at x=15, y=3
            "},
            None,
        );
        assert_eq!(part1(&file, 10).unwrap(), 26);
        assert_eq!(part2(&file, 20, 20).unwrap(), 56000011);
        drop(dir);
    }
}
