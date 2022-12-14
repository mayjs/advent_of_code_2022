use advent_of_code_2022::stream_file_blocks;
use anyhow::Result;
use std::{path::Path, str::FromStr};
use thiserror::Error;

const INPUT: &str = "input/day13.txt";

#[derive(Debug, Clone, PartialEq, Eq)]
enum Packet {
    Value(usize),
    List(Vec<Packet>),
}

#[derive(Error, Debug)]
enum PacketParseError {
    #[error("Invalid starting character: {0}")]
    InvalidStart(String),
    #[error("Invalid input character: {0}")]
    InvalidInputCharacter(String),
    #[error("Non terminated input: {0}")]
    NonTerminatedInput(String),
}

impl FromStr for Packet {
    type Err = PacketParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parse_stack: Vec<Vec<Packet>> = Vec::new();
        parse_stack.push(Vec::new());

        let mut current_number = String::new();
        let mut input = s.chars();
        if input.next() != Some('[') {
            return Err(PacketParseError::InvalidStart(s.to_string()));
        } else {
            for c in input {
                if c == ']' || c == ',' || c == '[' {
                    if !current_number.is_empty() {
                        parse_stack
                            .last_mut()
                            .unwrap()
                            .push(Packet::Value(current_number.parse().unwrap()));
                        current_number.clear();
                    }
                }
                if c.is_digit(10) {
                    current_number.push(c);
                } else if c == ']' {
                    let packet = parse_stack.pop().unwrap();
                    if let Some(parent) = parse_stack.last_mut() {
                        parent.push(Packet::List(packet));
                    } else {
                        return Ok(Packet::List(packet));
                    }
                } else if c == '[' {
                    parse_stack.push(Vec::new());
                } else if c != ',' {
                    return Err(PacketParseError::InvalidInputCharacter(s.to_string()));
                }
            }
        }

        Err(PacketParseError::NonTerminatedInput(s.to_string()))
    }
}

impl Packet {
    fn as_list(self) -> Self {
        match self {
            Packet::Value(v) => Packet::List(vec![Packet::Value(v)]),
            Packet::List(_) => self,
        }
    }

    fn is_list(&self) -> bool {
        match self {
            Packet::Value(_) => false,
            Packet::List(_) => true,
        }
    }

    fn get_children(&self) -> Option<&Vec<Self>> {
        match self {
            Packet::Value(_) => None,
            Packet::List(l) => Some(l),
        }
    }

    fn get_value(&self) -> Option<usize> {
        match self {
            Packet::Value(v) => Some(*v),
            Packet::List(_) => None,
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.is_list() ^ other.is_list() {
            // "If exactly one value is an integer, convert the integer to a list which contains that integer
            // as its only value, then retry the comparison."
            // This is pretty inefficient due to the clone calls, but it works.
            self.clone().as_list().cmp(&other.clone().as_list())
        } else if self.is_list() && other.is_list() {
            // If both values are lists, do a normal list comparison
            self.get_children()
                .unwrap()
                .cmp(other.get_children().unwrap())
        } else {
            // If both values are integers, compare them numerically
            self.get_value().unwrap().cmp(&other.get_value().unwrap())
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    Ok(stream_file_blocks(input)?
        .map(|pair| {
            (
                pair[0].parse::<Packet>().unwrap(),
                pair[1].parse::<Packet>().unwrap(),
            )
        })
        .enumerate()
        .filter(|(_, (a, b))| a < b)
        .map(|(i, _)| i + 1)
        .sum())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut incoming_data = stream_file_blocks(input)?
        .flat_map(|pair| pair.iter().map(|p| p.parse().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<Packet>>();
    // Add divider packets
    let divider_packets = vec!["[[2]]".parse()?, "[[6]]".parse()?];
    incoming_data.append(&mut divider_packets.clone());

    incoming_data.sort();

    Ok(incoming_data
        .iter()
        .enumerate()
        .filter(|(_, packet)| divider_packets.iter().any(|divider| &divider == packet))
        .map(|(i, _)| i + 1)
        .product())
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
    fn test_d13_examples() {
        let (dir, file) = create_example_file(
            indoc! {"
                [1,1,3,1,1]
                [1,1,5,1,1]

                [[1],[2,3,4]]
                [[1],4]

                [9]
                [[8,7,6]]

                [[4,4],4,4]
                [[4,4],4,4,4]

                [7,7,7,7]
                [7,7,7]

                []
                [3]

                [[[]]]
                [[]]

                [1,[2,[3,[4,[5,6,7]]]],8,9]
                [1,[2,[3,[4,[5,6,0]]]],8,9]
            "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 13);
        assert_eq!(part2(&file).unwrap(), 140);
        drop(dir);
    }
}
