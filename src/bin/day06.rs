use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::prelude::*,
    io::BufReader,
    path::Path,
};

use anyhow::Result;

const INPUT: &str = "input/day06.txt";

fn find_start_of_entity_marker(
    marker_len: usize,
    mut input: impl Iterator<Item = u8>,
) -> Option<(usize, impl Iterator<Item = u8>)> {
    let mut last_n_chars = VecDeque::new();
    input
        .by_ref()
        .take_while(|c| {
            if last_n_chars.len() >= marker_len {
                last_n_chars.pop_front();
            }
            last_n_chars.push_back(*c);
            // TODO: Use a bitset here
            last_n_chars.iter().collect::<HashSet<_>>().len() != marker_len
        })
        .enumerate()
        .last()
        // Adjust for 1-offset and the one skipped byte from take_while
        .map(|(idx, _)| (idx + 2, input))
}

fn run<P: AsRef<Path>>(marker_len: usize, input: P) -> Result<usize> {
    let file = BufReader::new(File::open(input)?);
    let input_bytes = file.bytes().map(|mb| mb.expect("Reading failed"));
    let (idx, _) =
        find_start_of_entity_marker(marker_len, input_bytes).expect("Found no SOP marker");
    Ok(idx)
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    run(4, input)
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    run(14, input)
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

    #[test]
    fn test_d05_examples() {
        let (dir, file) = create_example_file("mjqjpqmgbljsphdztnvjfqwrcgsmlb", None);
        assert_eq!(part1(&file).unwrap(), 7);
        assert_eq!(part2(&file).unwrap(), 19);
        drop(dir);
    }
}
