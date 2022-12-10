use advent_of_code_2022::stream_items_from_file;
use std::{
    collections::HashSet,
    num::ParseIntError,
    path::Path,
    str::FromStr,
};
use thiserror::Error;

use anyhow::Result;

const INPUT: &str = "input/day09.txt";

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownLeft,
    DownRight,
}

#[derive(Debug, Error)]
enum DirectionParseError {
    #[error("Invalid Direction '{0}'")]
    InvalidDirection(String),
}

impl FromStr for Direction {
    type Err = DirectionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(DirectionParseError::InvalidDirection(s.to_string())),
        }
    }
}

impl Direction {
    fn apply(&self, loc: &Location) -> Location {
        match self {
            Direction::Up => Location(loc.0, loc.1 + 1),
            Direction::Down => Location(loc.0, loc.1 - 1),
            Direction::Left => Location(loc.0 - 1, loc.1),
            Direction::Right => Location(loc.0 + 1, loc.1),
            Direction::UpRight => Location(loc.0 + 1, loc.1 + 1),
            Direction::UpLeft => Location(loc.0 - 1, loc.1 + 1),
            Direction::DownLeft => Location(loc.0 - 1, loc.1 - 1),
            Direction::DownRight => Location(loc.0 + 1, loc.1 - 1),
        }
    }
}

struct MovementInstruction(Direction, usize);

#[derive(Debug, Error)]
enum MovementInstructionParseError {
    #[error("Invalid movement '{0}'")]
    InvalidMovement(String),
    #[error("Could not parse direction")]
    DirectionError(#[from] DirectionParseError),
    #[error("Could not parse distance")]
    InvalidDistance(#[from] ParseIntError),
}

impl FromStr for MovementInstruction {
    type Err = MovementInstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(' ')
            .ok_or_else(|| MovementInstructionParseError::InvalidMovement(s.to_string()))
            .and_then(|(dir, dist)| Ok(MovementInstruction(dir.parse()?, dist.parse()?)))
    }
}

impl MovementInstruction {
    fn unfold(self) -> Vec<Direction> {
        vec![self.0; self.1]
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
struct Location(isize, isize);

impl Location {
    fn touches(&self, other: &Self) -> bool {
        self.0.abs_diff(other.0) <= 1 && self.1.abs_diff(other.1) <= 1
    }
}

fn simulate_movement(
    mut input: impl Iterator<Item = Direction>,
) -> impl Iterator<Item = (Location, Location, Option<Direction>)> {
    itertools::unfold(
        (Default::default(), Default::default()),
        move |(head, tail)| {
            input.next().map(|ins| {
                let next_head = ins.apply(head);
                let (next_tail, movements) = if !next_head.touches(tail) {
                    let dx = (next_head.0 - tail.0).clamp(-1, 1);
                    let dy = (next_head.1 - tail.1).clamp(-1, 1);

                    let dir = match (dx, dy) {
                        (1, 0) => Direction::Right,
                        (1, 1) => Direction::UpRight,
                        (0, 1) => Direction::Up,
                        (-1, 1) => Direction::UpLeft,
                        (-1, 0) => Direction::Left,
                        (-1, -1) => Direction::DownLeft,
                        (0, -1) => Direction::Down,
                        (1, -1) => Direction::DownRight,
                        _ => panic!("Must not get here"),
                    };
                    (Location(tail.0 + dx, tail.1 + dy), Some(dir))
                } else {
                    (tail.clone(), None)
                };

                *head = next_head.clone();
                *tail = next_tail.clone();
                (next_head, next_tail, movements)
            })
        },
    )
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    Ok(simulate_movement(
        stream_items_from_file::<P, MovementInstruction>(input)?
            .map(|mmi| mmi.expect("Invalid movement in input"))
            .flat_map(|i| i.unfold()),
    )
    .map(|(_, tail, _)| tail)
    .collect::<HashSet<_>>()
    .len())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let input_instructions = stream_items_from_file::<P, MovementInstruction>(input)?
        .map(|mmi| mmi.expect("Invalid movement in input"))
        .flat_map(|i| i.unfold());
    let mut tail_visited =
        simulate_movement((0..9).fold::<Box<dyn Iterator<Item = Direction>>, _>(
            Box::new(input_instructions),
            |ins, _| Box::new(simulate_movement(ins).flat_map(|(_, _, i)| i)),
        ))
        .map(|(h, _, _)| h)
        .collect::<HashSet<_>>();

    tail_visited.insert(Location(0, 0));

    Ok(tail_visited.len())
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
    fn test_d09_examples() {
        let (dir, file) = create_example_file(
            indoc! {"
                R 4
                U 4
                L 3
                D 1
                R 4
                D 1
                L 5
                R 2
            "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 13);
        assert_eq!(part2(&file).unwrap(), 1);
        drop(dir);
    }

    #[test]
    fn test_d09_p2_example() {
        let (dir, file) = create_example_file(
            indoc! {"
                R 5
                U 8
                L 8
                D 3
                R 17
                D 10
                L 25
                U 20
            "},
            None,
        );
        assert_eq!(part2(&file).unwrap(), 36);
        drop(dir);
    }
}
