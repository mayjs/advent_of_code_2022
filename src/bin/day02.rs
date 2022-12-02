use advent_of_code_2022::stream_items_from_file;
use std::{path::Path, str::FromStr};
use thiserror::Error;

use anyhow::Result;

const INPUT: &str = "input/day02.txt";

#[derive(Error, Debug)]
enum RockPaperScissorsError {
    #[error("Invalid shape symbol '{0}'")]
    InvalidShapeSymbol(String),
    #[error("Invalid strategy descriptor '{0}'")]
    InvalidStrategyDescriptor(String),
    #[error("Invalid Game goal symbol '{0}'")]
    InvalidGameGoalSymbol(String),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn base_score(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn can_beat(&self, other: &Self) -> bool {
        match self {
            Shape::Rock => other == &Shape::Scissors,
            Shape::Paper => other == &Shape::Rock,
            Shape::Scissors => other == &Shape::Paper,
        }
    }

    fn beaten_by(&self) -> Self {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn beats(&self) -> Self {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }
}

impl FromStr for Shape {
    type Err = RockPaperScissorsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(RockPaperScissorsError::InvalidShapeSymbol(s.to_string())),
        }
    }
}

struct GamePrediction(Shape, Shape);

impl FromStr for GamePrediction {
    type Err = RockPaperScissorsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(" ")
            .ok_or_else(|| RockPaperScissorsError::InvalidStrategyDescriptor(s.to_string()))
            .and_then(|(opponent, me)| Ok(GamePrediction(opponent.parse()?, me.parse()?)))
    }
}

impl GamePrediction {
    fn outcome_score(&self) -> usize {
        if self.0 == self.1 {
            3
        } else if self.1.can_beat(&self.0) {
            6
        } else {
            0
        }
    }

    fn score(&self) -> usize {
        self.1.base_score() + self.outcome_score()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum GameGoal {
    Lose,
    Draw,
    Win,
}

impl FromStr for GameGoal {
    type Err = RockPaperScissorsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameGoal::Lose),
            "Y" => Ok(GameGoal::Draw),
            "Z" => Ok(GameGoal::Win),
            _ => Err(RockPaperScissorsError::InvalidGameGoalSymbol(s.to_string())),
        }
    }
}

struct Strategy(Shape, GameGoal);

impl FromStr for Strategy {
    type Err = RockPaperScissorsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(" ")
            .ok_or_else(|| RockPaperScissorsError::InvalidStrategyDescriptor(s.to_string()))
            .and_then(|(opponent, goal)| Ok(Self(opponent.parse()?, goal.parse()?)))
    }
}

impl Strategy {
    fn to_game_prediction(&self) -> GamePrediction {
        GamePrediction(self.0, match self.1 {
            GameGoal::Lose => self.0.beats(),
            GameGoal::Draw => self.0,
            GameGoal::Win => self.0.beaten_by(),
        })
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    Ok(stream_items_from_file::<P, GamePrediction>(input)?
        .map(|g| g.expect("Invalid game").score())
        .sum())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    Ok(stream_items_from_file::<P, Strategy>(input)?
       .map(|s| s.expect("Invalid strategy").to_game_prediction().score())
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
            A Y
            B X
            C Z
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 15);
        assert_eq!(part2(&file).unwrap(), 12);
        drop(dir);
    }
}
