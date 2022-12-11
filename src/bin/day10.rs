use advent_of_code_2022::stream_items_from_file;
use anyhow::Result;
use std::{num::ParseIntError, path::Path, str::FromStr};
use thiserror::Error;

const INPUT: &str = "input/day10.txt";

#[derive(Debug, Clone)]
enum Instruction {
    NoOp,
    AddX(i64),
}

#[derive(Error, Debug)]
enum InstructionParseError {
    #[error("Invalid OpCode in this line: '{0}'")]
    InvalidOpCode(String),
    #[error("Missing parameter in this line: '{0}'")]
    MissingParam(String),
    #[error("Invalid parameter value")]
    InvalidIntegerParam(#[from] ParseIntError),
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "noop" {
            Ok(Self::NoOp)
        } else if s.starts_with("addx ") {
            s.split_once(' ')
                .ok_or_else(|| InstructionParseError::MissingParam(s.to_string()))
                .and_then(|(_, arg)| Ok(Self::AddX(arg.parse()?)))
        } else {
            Err(InstructionParseError::InvalidOpCode(s.to_string()))
        }
    }
}

impl Instruction {
    fn get_cycles(&self) -> usize {
        match self {
            Instruction::NoOp => 1,
            Instruction::AddX(_) => 2,
        }
    }

    fn run(&self, x: i64) -> (i64, usize) {
        let cycles = self.get_cycles();
        let new_x = match self {
            Instruction::NoOp => x,
            Instruction::AddX(v) => x + v,
        };

        (new_x, cycles)
    }
}

fn run_program(mut input: impl Iterator<Item = Instruction>) -> impl Iterator<Item = i64> {
    itertools::unfold(1, move |x| {
        input.next().map(|instruction| {
            let (new_x, cycles) = instruction.run(*x);
            let out = vec![*x; cycles];
            *x = new_x;
            out
        })
    })
    .flatten()
}

fn draw_crt(register_states: impl Iterator<Item = i64>) -> String {
    register_states
        .enumerate()
        .flat_map(|(step, x)| {
            let c = if (x - 1..=x + 1).contains(&((step % 40) as i64)) {
                '#'
            } else {
                '.'
            };

            if (step + 1) % 40 == 0 {
                vec![c, '\n']
            } else {
                vec![c]
            }
        })
        .collect()
}

fn part1<P: AsRef<Path>>(input: P) -> Result<i64> {
    Ok(run_program(
        stream_items_from_file::<P, Instruction>(input)?
            .map(|mi| mi.expect("Unparseable instruction")),
    )
    .enumerate()
    .filter(|(step, _)| {
        let rstep = step + 1;
        rstep == 20 || (rstep >= 60 && ((rstep - 20) % 40 == 0))
    })
    .map(|(step, x)| ((step + 1) as i64) * x)
    .sum())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<String> {
    Ok(draw_crt(run_program(
        stream_items_from_file::<P, Instruction>(input)?
            .map(|mi| mi.expect("Unparseable instruction")),
    )))
}

fn main() -> Result<()> {
    println!("Answer for part 1: {}", part1(INPUT)?);
    println!("Answer for part 2:\n{}", part2(INPUT)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_of_code_2022::test_helpers::create_example_file;
    use indoc::indoc;

    #[test]
    fn test_d10_examples() {
        let (dir, file) = create_example_file(
            indoc! {"
                addx 15
                addx -11
                addx 6
                addx -3
                addx 5
                addx -1
                addx -8
                addx 13
                addx 4
                noop
                addx -1
                addx 5
                addx -1
                addx 5
                addx -1
                addx 5
                addx -1
                addx 5
                addx -1
                addx -35
                addx 1
                addx 24
                addx -19
                addx 1
                addx 16
                addx -11
                noop
                noop
                addx 21
                addx -15
                noop
                noop
                addx -3
                addx 9
                addx 1
                addx -3
                addx 8
                addx 1
                addx 5
                noop
                noop
                noop
                noop
                noop
                addx -36
                noop
                addx 1
                addx 7
                noop
                noop
                noop
                addx 2
                addx 6
                noop
                noop
                noop
                noop
                noop
                addx 1
                noop
                noop
                addx 7
                addx 1
                noop
                addx -13
                addx 13
                addx 7
                noop
                addx 1
                addx -33
                noop
                noop
                noop
                addx 2
                noop
                noop
                noop
                addx 8
                noop
                addx -1
                addx 2
                addx 1
                noop
                addx 17
                addx -9
                addx 1
                addx 1
                addx -3
                addx 11
                noop
                noop
                addx 1
                noop
                addx 1
                noop
                noop
                addx -13
                addx -19
                addx 1
                addx 3
                addx 26
                addx -30
                addx 12
                addx -1
                addx 3
                addx 1
                noop
                noop
                noop
                addx -9
                addx 18
                addx 1
                addx 2
                noop
                noop
                addx 9
                noop
                noop
                noop
                addx -1
                addx 2
                addx -37
                addx 1
                addx 3
                noop
                addx 15
                addx -21
                addx 22
                addx -6
                addx 1
                noop
                addx 2
                addx 1
                noop
                addx -10
                noop
                noop
                addx 20
                addx 1
                addx 2
                addx 2
                addx -6
                addx -11
                noop
                noop
                noop
            "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 13140);

        let expected_output = indoc! {"
            ##..##..##..##..##..##..##..##..##..##..
            ###...###...###...###...###...###...###.
            ####....####....####....####....####....
            #####.....#####.....#####.....#####.....
            ######......######......######......####
            #######.......#######.......#######.....
        "};
        assert_eq!(part2(&file).unwrap(), expected_output);
        drop(dir);
    }

    #[test]
    fn test_simple_prog() {
        let (dir, file) = create_example_file(
            indoc! {"
                noop
                addx 3
                addx -5
            "},
            None,
        );

        let out_states = run_program(
            stream_items_from_file::<_, Instruction>(file)
                .unwrap()
                .map(|mi| mi.unwrap()),
        )
        .collect::<Vec<_>>();

        assert_eq!(out_states, vec![1, 1, 1, 4, 4]);
        drop(dir);
    }
}
