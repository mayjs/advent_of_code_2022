use std::{fs::File, io::prelude::*, io::BufReader, path::Path};

use anyhow::Result;

// FIXME: This was written in a rush, lots of copy-pasted code between functions, no error handling etc.

const INPUT: &str = "input/day05.txt";

struct StacksOfCrates(Vec<Vec<char>>);

impl StacksOfCrates {
    fn parse(input: Vec<String>) -> Self {
        let number_of_stacks = input
            .iter()
            .rev()
            .next()
            .expect("Can't have empty input")
            .chars()
            .filter(|c| *c == '[')
            .count();

        let mut result = vec![Vec::new(); number_of_stacks];

        for line in input.iter().rev() {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '[')
                .for_each(|(i, _)| {
                    let stack_idx = i / 4;
                    result[stack_idx].push(line.chars().nth(i + 1).expect("Missing char"));
                });
        }

        StacksOfCrates(result)
    }
}

struct RestackingInstruction(usize, usize, usize);

impl RestackingInstruction {
    fn parse_from_str(s: &str) -> Self {
        let parts = s.splitn(6, ' ').collect::<Vec<_>>();
        RestackingInstruction(
            parts[1].parse().unwrap(),
            parts[3].parse().unwrap(),
            parts[5].parse().unwrap(),
        )
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<String> {
    let mut input_lines = BufReader::new(File::open(input)?)
        .lines()
        .map(|ml| ml.expect("Could not read"));

    let mut stacks = StacksOfCrates::parse(
        input_lines
            .by_ref()
            .take_while(|l| l.chars().nth(1).unwrap() != '1')
            .collect(),
    );

    input_lines.next();
    for ins in input_lines.map(|l| RestackingInstruction::parse_from_str(&l)) {
        for _ in 0..ins.0 {
            let out = stacks.0[ins.1 - 1]
                .pop()
                .expect("Could not follow move instruction");
            stacks.0[ins.2 - 1].push(out);
        }
    }

    Ok(stacks
        .0
        .iter()
        .map(|s| s.last().expect("Could not read top of stack"))
        .collect::<String>())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<String> {
    let mut input_lines = BufReader::new(File::open(input)?)
        .lines()
        .map(|ml| ml.expect("Could not read"));

    let mut stacks = StacksOfCrates::parse(
        input_lines
            .by_ref()
            .take_while(|l| l.chars().nth(1).unwrap() != '1')
            .collect(),
    );

    input_lines.next();
    for ins in input_lines.map(|l| RestackingInstruction::parse_from_str(&l)) {
        let popped = (0..ins.0)
            .map(|_| {
                stacks.0[ins.1 - 1]
                    .pop()
                    .expect("Could not follow move instruction")
            })
            .collect::<Vec<_>>();
        popped.into_iter().rev().for_each(|c| stacks.0[ins.2 - 1].push(c));
    }

    Ok(stacks
        .0
        .iter()
        .map(|s| s.last().expect("Could not read top of stack"))
        .collect::<String>())
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
                [D]    
            [N] [C]    
            [Z] [M] [P]
             1   2   3 

            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
        "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), "CMZ");
        assert_eq!(part2(&file).unwrap(), "MCD");
        drop(dir);
    }
}
