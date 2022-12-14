use advent_of_code_2022::stream_items_from_file;
use anyhow::Result;
use std::{cmp, collections::HashSet, num::ParseIntError, path::Path, str::FromStr};
use thiserror::Error;

const INPUT: &str = "input/day14.txt";

type Coord = (isize, isize);

#[derive(Debug, Clone)]
struct Line(Vec<Coord>);

#[derive(Error, Debug)]
enum LineParseError {
    #[error("Invalid pair")]
    InvalidPair,
    #[error("Invalid number")]
    InvalidNumber(#[from] ParseIntError),
}

impl FromStr for Line {
    type Err = LineParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Line(
            s.split("->")
                .map(|p| {
                    p.trim()
                        .split_once(',')
                        .ok_or(LineParseError::InvalidPair)
                        .and_then(|(x, y)| Ok((x.parse()?, y.parse()?)))
                })
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl Line {
    fn get_points(&self) -> impl Iterator<Item = Coord> + '_ {
        self.0
            .iter()
            .zip(self.0[1..].iter())
            .flat_map(move |(segment_start, segment_end)| {
                if segment_start.1 == segment_end.1 {
                    let start = cmp::min(segment_start.0, segment_end.0);
                    let end = cmp::max(segment_start.0, segment_end.0);
                    (start..=end)
                        .map(|x| (x, segment_start.1))
                        .collect::<Vec<_>>()
                } else {
                    let start = cmp::min(segment_start.1, segment_end.1);
                    let end = cmp::max(segment_start.1, segment_end.1);
                    (start..=end)
                        .map(|y| (segment_start.0, y))
                        .collect::<Vec<_>>()
                }
            })
    }
}

fn drop_sand_bottomless(environment: &HashSet<Coord>, start: Coord) -> Option<Coord> {
    let lowest_point = *environment.iter().map(|(_, y)| y).max().unwrap();
    let mut sand_pos = start.clone();
    while sand_pos.1 < lowest_point {
        if !environment.contains(&(sand_pos.0, sand_pos.1 + 1)) {
            sand_pos = (sand_pos.0, sand_pos.1 + 1);
        } else if !environment.contains(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
            sand_pos = (sand_pos.0 - 1, sand_pos.1 + 1);
        } else if !environment.contains(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
            sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
        } else {
            return Some(sand_pos);
        }
    }

    None
}

fn drop_sand_with_floor(environment: &HashSet<Coord>, start: Coord, floor: isize) -> Coord {
    let mut sand_pos = start.clone();
    while sand_pos.1 < floor - 1 {
        if !environment.contains(&(sand_pos.0, sand_pos.1 + 1)) {
            sand_pos = (sand_pos.0, sand_pos.1 + 1);
        } else if !environment.contains(&(sand_pos.0 - 1, sand_pos.1 + 1)) {
            sand_pos = (sand_pos.0 - 1, sand_pos.1 + 1);
        } else if !environment.contains(&(sand_pos.0 + 1, sand_pos.1 + 1)) {
            sand_pos = (sand_pos.0 + 1, sand_pos.1 + 1);
        } else {
            return sand_pos;
        }
    }
    sand_pos
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut rocks = stream_items_from_file::<P, Line>(input)?
        .map(|ml| ml.unwrap())
        .flat_map(|l| l.get_points().collect::<Vec<_>>())
        .collect::<HashSet<_>>();
    let mut dropped = 0;
    loop {
        match drop_sand_bottomless(&rocks, (500, 0)) {
            Some(p) => {
                rocks.insert(p);
                dropped += 1;
            }
            None => return Ok(dropped),
        }
    }
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut rocks = stream_items_from_file::<P, Line>(input)?
        .map(|ml| ml.unwrap())
        .flat_map(|l| l.get_points().collect::<Vec<_>>())
        .collect::<HashSet<_>>();
    let mut dropped = 0;
    let lowest_rock = *rocks.iter().map(|(_, y)| y).max().unwrap();
    loop {
        let pos = drop_sand_with_floor(&rocks, (500, 0), lowest_rock + 2);
        dropped += 1;
        if pos == (500, 0) {
            return Ok(dropped);
        } else {
            rocks.insert(pos);
        }
    }
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
    fn test_d14_examples() {
        let (dir, file) = create_example_file(
            indoc! {"
                498,4 -> 498,6 -> 496,6
                503,4 -> 502,4 -> 502,9 -> 494,9
            "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 24);
        assert_eq!(part2(&file).unwrap(), 93);
        drop(dir);
    }
}
