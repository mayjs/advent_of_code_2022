use std::path::Path;

use advent_of_code_2022::stream_file_blocks;
use anyhow::Result;

const INPUT: &str = "input/day01.txt";

fn get_elf_calories_stream<P: AsRef<Path>>(input: P) -> Result<impl Iterator<Item=usize>> {
    Ok(stream_file_blocks(input)?.map(|elf_list| {
        elf_list.into_iter().map(|cal_count| {
            cal_count.parse::<usize>().expect("Invalid input")
        }).sum()
    }))
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    Ok(get_elf_calories_stream(input)?.max().unwrap_or_default())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut current_max_values = Vec::new();

    for calories in get_elf_calories_stream(input)? {
        let insert_pos = match current_max_values.binary_search(&calories) {
            Ok(pos) => pos,
            Err(pos) => pos
        };
        if current_max_values.len() < 3 || insert_pos != 0 {
            current_max_values.insert(insert_pos, calories);
            if current_max_values.len() > 3 {
                current_max_values.remove(0);
            }
        }
    }

    Ok(current_max_values.into_iter().sum())
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
    fn test_d01_examples() {
        let (dir, file) = create_example_file(indoc!{"
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
        "}, None);
        assert_eq!(part1(&file).unwrap(), 24000);
        assert_eq!(part2(&file).unwrap(), 45000);
        drop(dir);
    }
}
