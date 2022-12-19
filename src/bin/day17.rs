use advent_of_code_2022::stream_items_from_file;
use anyhow::anyhow;
use anyhow::Result;
use std::collections::HashMap;
use std::{collections::HashSet, path::Path, str::FromStr};

const INPUT: &str = "input/day17.txt";

type Coordinate = (usize, usize);

#[derive(Debug, Clone)]
struct Rock<'a>(&'a [Coordinate], usize);

const ROCK_TYPES: [Rock<'static>; 5] = [
    Rock(&[(0, 0), (1, 0), (2, 0), (3, 0)], 0),
    Rock(&[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], 1),
    Rock(&[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], 2),
    Rock(&[(0, 0), (0, 1), (0, 2), (0, 3)], 3),
    Rock(&[(0, 0), (1, 0), (0, 1), (1, 1)], 4),
];

fn get_rock_types_iteration() -> impl Iterator<Item = &'static Rock<'static>> {
    std::iter::repeat(())
        .enumerate()
        .map(|(i, _)| &ROCK_TYPES[i % ROCK_TYPES.len()])
}

impl<'a> Rock<'a> {
    fn iterate_rock_coords(&self) -> impl Iterator<Item = Coordinate> + 'a {
        self.0.iter().cloned()
    }

    fn check_collision(&self, other_rocks: &HashSet<Coordinate>, offset: Coordinate) -> bool {
        self.iterate_rock_coords()
            .map(|c| (c.0 + offset.0, c.1 + offset.1))
            .any(|c| other_rocks.contains(&c))
    }
}

type CaveState = HashSet<Coordinate>;

#[derive(Debug, Clone, Copy)]
enum JetDirection {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct JetPattern(Vec<JetDirection>);

impl FromStr for JetPattern {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .map(|c| match c {
                '<' => Ok(JetDirection::Left),
                '>' => Ok(JetDirection::Right),
                _ => Err(anyhow!("Unexpected input direction {}", c)),
            })
            .collect::<Result<Vec<_>>>()
            .map(JetPattern)
    }
}

impl JetPattern {
    fn into_iter(self) -> impl Iterator<Item = JetDirection> {
        self.0.into_iter().cycle()
    }

    fn into_iter_pattern_idx(self) -> impl Iterator<Item = (usize, JetDirection)> {
        self.0.into_iter().enumerate().cycle()
    }
}

fn drop_rock(
    cave_state: &mut CaveState,
    rock: &Rock,
    jets: &mut impl Iterator<Item = JetDirection>,
) {
    let mut rock_position = (2, cave_state.iter().map(|c| c.1 + 1).max().unwrap_or(0) + 3);

    let rock_width = rock.0.iter().map(|c| c.0).max().unwrap() + 1;

    loop {
        // 1.: Hot jets push the rock
        match jets.next().unwrap() {
            JetDirection::Left => {
                if rock_position.0 > 0
                    && !rock.check_collision(cave_state, (rock_position.0 - 1, rock_position.1))
                {
                    rock_position.0 -= 1;
                } else {
                }
            }
            JetDirection::Right => {
                if rock_position.0 + rock_width < 7
                    && !rock.check_collision(cave_state, (rock_position.0 + 1, rock_position.1))
                {
                    rock_position.0 += 1;
                } else {
                }
            }
        }

        // 2.: Rock falls 1 block
        if rock_position.1 == 0
            || rock.check_collision(cave_state, (rock_position.0, rock_position.1 - 1))
        {
            // We hit something, stop here
            rock.iterate_rock_coords()
                .map(|c| (rock_position.0 + c.0, rock_position.1 + c.1))
                .for_each(|p| {
                    cave_state.insert(p);
                });
            break;
        } else {
            rock_position.1 -= 1;
        }
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let mut jet_pattern = stream_items_from_file::<_, JetPattern>(input)?
        .map(|mi| mi.unwrap())
        .next()
        .unwrap()
        .into_iter();
    let mut cave_state = HashSet::default();
    get_rock_types_iteration().take(2022).for_each(|rock| {
        drop_rock(&mut cave_state, rock, &mut jet_pattern);
    });

    let height = cave_state.iter().map(|c| c.1).max().unwrap() + 1;

    Ok(height)
}

// The higher the better the reliability
const FINGERPRINT_LENGTH: usize = 20;
type TopRockFingerprint = [[bool; 7]; FINGERPRINT_LENGTH];

#[derive(Debug, Clone, PartialEq, Eq)]
struct Fingerprint {
    rock: usize,
    jet: usize,
    last_rows: TopRockFingerprint,
    max_y: usize,
    n_rocks: usize,
}

impl Fingerprint {
    fn build(
        number_of_rocks: usize,
        rock: &Rock,
        jet: usize,
        cave_state: &HashSet<Coordinate>,
    ) -> Self {
        let max_y = cave_state.iter().map(|c| c.1).max().unwrap();
        let mut last_rows = TopRockFingerprint::default();

        cave_state
            .iter()
            .filter(|c| c.1 + FINGERPRINT_LENGTH > max_y)
            .for_each(|c| {
                last_rows[max_y - c.1][c.0] = true;
            });

        Fingerprint {
            rock: rock.1,
            jet,
            last_rows,
            max_y,
            n_rocks: number_of_rocks,
        }
    }

    fn matches(&self, other: &Self) -> bool {
        self.rock == other.rock && self.jet == other.jet && self.last_rows == other.last_rows
    }
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let jet_pattern = stream_items_from_file::<_, JetPattern>(input)?
        .map(|mi| mi.unwrap())
        .next()
        .unwrap();
    let mut jet_pattern = jet_pattern.into_iter_pattern_idx().peekable();
    let mut cave_state = HashSet::default();

    const ITERATIONS: usize = 1000000000000;

    let mut rock_sequence = get_rock_types_iteration().peekable();

    let mut fingerprint_store = HashMap::<(usize, usize), Vec<Fingerprint>>::new();

    let mut loop_parameters = None;

    let mut fallen_rocks: usize = 0;

    for rock in rock_sequence.by_ref() {
        let (jet_idx, _) = jet_pattern.peek().unwrap();
        if fallen_rocks > 20 {
            let fingerprint = Fingerprint::build(fallen_rocks, rock, *jet_idx, &cave_state);
            let entry = fingerprint_store.entry((rock.1, *jet_idx)).or_default();
            if let Some(matching_fingerprint) =
                entry.iter().filter(|f| f.matches(&fingerprint)).next()
            {
                println!(
                    "Loop identified (from rock dropped at {} to rock dropped at {})",
                    matching_fingerprint.n_rocks, fallen_rocks
                );
                loop_parameters = Some((
                    fallen_rocks - matching_fingerprint.n_rocks,
                    fingerprint.max_y - matching_fingerprint.max_y,
                    fingerprint,
                ));
                break;
            } else {
                entry.push(fingerprint);
            }
        }
        drop_rock(
            &mut cave_state,
            rock,
            &mut jet_pattern.by_ref().map(|(_, j)| j),
        );
        fallen_rocks += 1;
    }

    let (rocks_per_loop, height_per_loop, loop_fingerprint) = loop_parameters.unwrap();
    let repeat_loop_times = (ITERATIONS - fallen_rocks) / rocks_per_loop;
    fallen_rocks += repeat_loop_times * rocks_per_loop;
    println!("Loop gets us to {}", fallen_rocks);
    let loop_height = height_per_loop * repeat_loop_times;
    // TODO Unroll fingerprint top onto cave state and drop remaining rocks

    while rock_sequence.peek().unwrap().1 != loop_fingerprint.rock {
        rock_sequence.next();
    }

    rock_sequence
        .take(ITERATIONS - fallen_rocks)
        .for_each(|rock| {
            drop_rock(
                &mut cave_state,
                rock,
                &mut jet_pattern.by_ref().map(|(_, j)| j),
            );
        });

    let height = cave_state.iter().map(|c| c.1).max().unwrap() + 1 + loop_height;

    Ok(height)
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
                >>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>
            "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 3068);
        assert_eq!(part2(&file).unwrap(), 1514285714288);
        drop(dir);
    }
}
