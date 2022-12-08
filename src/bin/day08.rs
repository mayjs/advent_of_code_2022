use std::{
    cmp,
    fs::File,
    io::{BufRead, BufReader},
    iter,
    path::Path,
};
use thiserror::Error;

use anyhow::Result;

const INPUT: &str = "input/day08.txt";

#[derive(Debug)]
struct Field2D<T> {
    entries: Vec<T>,
    width: usize,
}

#[derive(Debug, Error)]
enum Field2dParseError {
    #[error("Empty input")]
    EmptyInput,
}

// TODO: Check if we could reuse the field impl from last year here
impl Field2D<u8> {
    fn from_lines(lines: impl Iterator<Item = String>) -> Result<Self, Field2dParseError> {
        let mut lines = lines.peekable();
        let width = lines.peek().ok_or(Field2dParseError::EmptyInput)?.len();
        let entries = lines
            .flat_map(|line| {
                line.chars()
                    // TODO: Don't panic
                    .map(|c| c.to_digit(10).expect("Could not parse") as u8)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Ok(Self { entries, width })
    }
}

impl<T> Field2D<T> {
    fn get(&self, x: usize, y: usize) -> Option<&T> {
        if x < self.width {
            let idx = y * self.width + x;
            self.entries.get(idx)
        } else {
            None
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.entries.len() / self.width
    }
}

type TreeMap = Field2D<u8>;

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let field = TreeMap::from_lines(
        BufReader::new(File::open(input)?)
            .lines()
            .map(|ml| ml.expect("Could not read line")),
    )?;

    let mut count = 2 * field.width() + 2 * field.height() - 4;
    for x in 1..field.width() - 1 {
        for y in 1..field.height() - 1 {
            let mut any_dir_visible = false;
            for dir in &[-1, 1] {
                any_dir_visible |= iter::repeat(x)
                    .enumerate()
                    .map(|(i, x)| (x as i32 + ((i + 1) as i32 * dir)))
                    .take_while(|nx| *nx >= 0 && *nx < field.width() as i32)
                    .map(|nx| nx as usize)
                    .all(|nx| field.get(nx, y).unwrap() < field.get(x, y).unwrap());
            }
            for dir in &[-1, 1] {
                any_dir_visible |= iter::repeat(y)
                    .enumerate()
                    .map(|(i, y)| (y as i32 + (i + 1) as i32 * dir))
                    .take_while(|ny| *ny >= 0 && *ny < field.height() as i32)
                    .map(|ny| ny as usize)
                    .all(|ny| field.get(x, ny).unwrap() < field.get(x, y).unwrap());
            }
            if any_dir_visible {
                count += 1
            }
        }
    }
    Ok(count)
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let field = TreeMap::from_lines(
        BufReader::new(File::open(input)?)
            .lines()
            .map(|ml| ml.expect("Could not read line")),
    )?;
    let mut max_score = 1;
    for x in 1..field.width() - 1 {
        for y in 1..field.height() - 1 {
            let mut score = 1;
            for dir in &[-1, 1] {
                let xr = iter::repeat(x)
                    .enumerate()
                    .map(|(i, x)| x as i32 + ((i + 1) as i32 * dir))
                    .take_while(|nx| {
                        *nx >= 1
                            && *nx < field.width() as i32 - 1
                            && field.get(*nx as usize, y).unwrap() < field.get(x, y).unwrap()
                    })
                    .count() + 1;
                let yr = iter::repeat(y)
                    .enumerate()
                    .map(|(i, y)| y as i32 + ((i + 1) as i32 * dir))
                    .take_while(|ny| {
                        *ny >= 1
                            && *ny < field.height() as i32 - 1
                            && field.get(x, *ny as usize).unwrap() < field.get(x, y).unwrap()
                    })
                    .count() + 1;

                score *= xr * yr;
            }
            max_score = cmp::max(score, max_score);
        }
    }
    Ok(max_score)
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
    fn test_d05_examples() {
        let (dir, file) = create_example_file(
            indoc! {"
            30373
            25512
            65332
            33549
            35390
            "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 21);
        assert_eq!(part2(&file).unwrap(), 8);
        drop(dir);
    }
}
