use advent_of_code_2022::stream_file_blocks;
use anyhow::Result;
use std::{num::ParseIntError, path::Path, str::FromStr};
use thiserror::Error;

const INPUT: &str = "input/day11.txt";

#[derive(Debug)]
struct ThrowTest {
    divisible_by: usize,
    if_true: usize,
    if_false: usize,
}

impl ThrowTest {
    fn run(&self, item: usize) -> Throw {
        Throw {
            item,
            to: if item % self.divisible_by == 0 {
                self.if_true
            } else {
                self.if_false
            },
        }
    }
}

#[derive(Debug, Error)]
enum ThrowTestParseError {
    #[error("Could not find a divisor")]
    NoDivisorError,
    #[error("Could not find the true case")]
    NoTrueCase,
    #[error("Could not find the false case")]
    NoFalseCase,
    #[error("Invalid number")]
    InvalidNumber(#[from] ParseIntError),
}

impl TryFrom<&[String]> for ThrowTest {
    type Error = ThrowTestParseError;

    fn try_from(value: &[String]) -> Result<Self, Self::Error> {
        Ok(ThrowTest {
            divisible_by: value[0]
                .rsplit_once(' ')
                .ok_or(ThrowTestParseError::NoDivisorError)?
                .1
                .parse()?,
            if_true: value[1]
                .rsplit_once(' ')
                .ok_or(ThrowTestParseError::NoTrueCase)?
                .1
                .parse()?,
            if_false: value[2]
                .rsplit_once(' ')
                .ok_or(ThrowTestParseError::NoFalseCase)?
                .1
                .parse()?,
        })
    }
}

#[derive(Debug)]
enum Operator {
    Add,
    Mult,
    Square,
}

#[derive(Debug)]
struct Operation {
    operator: Operator,
    operand: usize,
}

impl Operation {
    fn apply(&self, input: usize) -> usize {
        match self.operator {
            Operator::Add => input + self.operand,
            Operator::Mult => input * self.operand,
            Operator::Square => input * input,
        }
    }
}

#[derive(Debug, Error)]
enum OperationParseError {
    #[error("No operator in operation descriptor: '{0}'")]
    NoOperatorFound(String),
    #[error("Invalid operand")]
    InvalidOperand(#[from] ParseIntError),
}

impl FromStr for Operation {
    type Err = OperationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with("old * old") {
            Ok(Operation {
                operator: Operator::Square,
                operand: 0,
            })
        } else if let Some((_, operand)) = s.split_once('+') {
            Ok(Operation {
                operator: Operator::Add,
                operand: operand.trim().parse()?,
            })
        } else if let Some((_, operand)) = s.split_once('*') {
            Ok(Operation {
                operator: Operator::Mult,
                operand: operand.trim().parse()?,
            })
        } else {
            Err(OperationParseError::NoOperatorFound(s.to_string()))
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    throw_test: ThrowTest,
}

impl Monkey {
    fn take_turn(&mut self) -> Vec<Throw> {
        let throws = self
            .items
            .iter()
            .map(|item| {
                let new_worry_level = self.operation.apply(*item) / 3;
                self.throw_test.run(new_worry_level)
            })
            .collect();

        self.items.clear();

        throws
    }

    /// Do a turn of the monkey game, but don't reduce the worry level anymore.
    /// To prevent huge worry level numbers, we just store the worry level modulo the least common
    /// multiple of all throw test divisors
    fn take_turn_ring_op(&mut self, test_lcm: usize) -> Vec<Throw> {
        let throws = self
            .items
            .iter()
            .map(|item| {
                let new_worry_level = self.operation.apply(*item) % test_lcm;
                self.throw_test.run(new_worry_level)
            })
            .collect();

        self.items.clear();

        throws
    }
}

#[derive(Error, Debug)]
enum MonkeyParseError {
    #[error("Not enough lines in monkey descriptor")]
    NotEnoughLines,
    #[error("Invalid item descriptor line")]
    InvalidItemDescriptor,
    #[error("Invalid item number")]
    InvalidItemNumber(#[from] ParseIntError),
    #[error("Invalid operation")]
    InvalidOperation(#[from] OperationParseError),
    #[error("Invalid throw test")]
    InvalidThrowTest(#[from] ThrowTestParseError),
}

impl TryFrom<Vec<String>> for Monkey {
    type Error = MonkeyParseError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        if value.len() == 6 {
            let items = value[1]
                .split_once(":")
                .ok_or(MonkeyParseError::InvalidItemDescriptor)?
                .1
                .split(',')
                .map(|item| item.trim().parse())
                .collect::<Result<Vec<_>, _>>()?;
            let operation = value[2].parse()?;
            let throw_test = value[3..6].try_into()?;

            Ok(Monkey {
                items,
                operation,
                throw_test,
            })
        } else {
            Err(MonkeyParseError::NotEnoughLines)
        }
    }
}

struct Throw {
    item: usize,
    to: usize,
}

impl Throw {
    fn execute(self, monkeys: &mut Vec<Monkey>) {
        monkeys[self.to].items.push(self.item);
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut monkeys = stream_file_blocks(input)?
        .map::<Monkey, _>(|block| block.try_into().expect("Invalid monkey descriptor"))
        .collect::<Vec<_>>();
    let mut monkey_throw_counts = vec![0; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            let throws = monkeys[i].take_turn();
            monkey_throw_counts[i] += throws.len();
            throws
                .into_iter()
                .for_each(|throw| throw.execute(&mut monkeys));
        }
    }

    let (most_idx, &most_val) = monkey_throw_counts
        .iter()
        .enumerate()
        .max_by_key(|(_, v)| *v)
        .unwrap();
    monkey_throw_counts.swap_remove(most_idx);
    let second_most_val = monkey_throw_counts.iter().max().unwrap();

    Ok(most_val * second_most_val)
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut monkeys = stream_file_blocks(input)?
        .map::<Monkey, _>(|block| block.try_into().expect("Invalid monkey descriptor"))
        .collect::<Vec<_>>();

    let lcm = monkeys
        .iter()
        .map(|monkey| monkey.throw_test.divisible_by)
        .reduce(|x, y| num::integer::lcm(x, y))
        .unwrap();

    let mut monkey_throw_counts = vec![0; monkeys.len()];

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let throws = monkeys[i].take_turn_ring_op(lcm);
            monkey_throw_counts[i] += throws.len();
            throws
                .into_iter()
                .for_each(|throw| throw.execute(&mut monkeys));
        }
    }

    let (most_idx, &most_val) = monkey_throw_counts
        .iter()
        .enumerate()
        .max_by_key(|(_, v)| *v)
        .unwrap();
    monkey_throw_counts.swap_remove(most_idx);
    let second_most_val = monkey_throw_counts.iter().max().unwrap();

    Ok(most_val * second_most_val)
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
    fn test_d10_examples() {
        let (dir, file) = create_example_file(
            indoc! {"
                Monkey 0:
                  Starting items: 79, 98
                  Operation: new = old * 19
                  Test: divisible by 23
                    If true: throw to monkey 2
                    If false: throw to monkey 3

                Monkey 1:
                  Starting items: 54, 65, 75, 74
                  Operation: new = old + 6
                  Test: divisible by 19
                    If true: throw to monkey 2
                    If false: throw to monkey 0

                Monkey 2:
                  Starting items: 79, 60, 97
                  Operation: new = old * old
                  Test: divisible by 13
                    If true: throw to monkey 1
                    If false: throw to monkey 3

                Monkey 3:
                  Starting items: 74
                  Operation: new = old + 3
                  Test: divisible by 17
                    If true: throw to monkey 0
                    If false: throw to monkey 1
            "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 10605);
        assert_eq!(part2(&file).unwrap(), 2713310158);
        drop(dir);
    }
}
