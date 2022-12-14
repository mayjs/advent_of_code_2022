use advent_of_code_2022::{field2d::Field2D, stream_items_from_file};
use anyhow::Result;
use std::{collections::BinaryHeap,  path::Path };

const INPUT: &str = "input/day12.txt";

#[derive(Debug, Clone)]
struct Heightmap {
    start: (usize, usize),
    goal: (usize, usize),
    map: Field2D<usize>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Heightmap {
    fn from_lines(input: impl Iterator<Item = String>) -> Self {
        let mut start = None;
        let mut goal = None;

        let mut line_idx = 0;
        let map = Field2D::parse(input, |line| {
            let res = line
                .chars()
                .enumerate()
                .map(|(i, h)| {
                    if h == 'S' {
                        start = Some((i, line_idx));
                        1
                    } else if h == 'E' {
                        goal = Some((i, line_idx));
                        26
                    } else {
                        (h as u8 - b'a') as usize
                    }
                })
                .collect::<Vec<_>>();
            line_idx += 1;
            res
        })
        .unwrap();

        Self {
            start: start.unwrap(),
            goal: goal.unwrap(),
            map,
        }
    }

    fn path_search(&self) -> Option<usize> {
        // Dijkstra path search mostly taken from the rust binary heap documentation example
        let mut distances =
            Field2D::<usize>::new_with_value(self.map.width(), self.map.height(), usize::MAX);
        let mut heap = BinaryHeap::new();

        distances[self.start] = 0;
        heap.push(State {
            cost: 0,
            position: self.start,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if position == self.goal {
                return Some(cost);
            }

            if cost > distances[position] {
                continue;
            }

            for neighbor in self
                .map
                .neighbors(position.0, position.1)
                .filter(|neighbor| self.map[*neighbor] <= self.map[position] + 1)
            {
                let next = State {
                    cost: cost + 1,
                    position: neighbor,
                };

                if next.cost < distances[next.position] {
                    heap.push(next);
                    distances[next.position] = next.cost;
                }
            }
        }

        None
    }

    fn find_all_distances_to_goal(&self) -> Field2D<usize> {
        // Dijkstra path search mostly taken from the rust binary heap documentation example
        let mut distances =
            Field2D::<usize>::new_with_value(self.map.width(), self.map.height(), usize::MAX);
        let mut heap = BinaryHeap::new();

        distances[self.goal] = 0;
        heap.push(State {
            cost: 0,
            position: self.goal,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if cost > distances[position] {
                continue;
            }

            for neighbor in self
                .map
                .neighbors(position.0, position.1)
                .filter(|neighbor| self.map[*neighbor] + 1 >= self.map[position])
            {
                let next = State {
                    cost: cost + 1,
                    position: neighbor,
                };

                if next.cost < distances[next.position] {
                    heap.push(next);
                    distances[next.position] = next.cost;
                }
            }
        }

        distances
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let map = Heightmap::from_lines(stream_items_from_file(input)?.map(|i| i.unwrap()));
    Ok(map.path_search().unwrap())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let map = Heightmap::from_lines(stream_items_from_file(input)?.map(|i| i.unwrap()));
    let distances = map.find_all_distances_to_goal();
    Ok(*distances
        .iter_with_position()
        .filter(|(pos, _)| map.map[*pos] == 0)
        .min_by_key(|(_, dist)| *dist)
        .unwrap().1)
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
    fn test_d12_examples() {
        let (dir, file) = create_example_file(
            indoc! {"
                Sabqponm
                abcryxxl
                accszExk
                acctuvwj
                abdefghi
            "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 31);
        assert_eq!(part2(&file).unwrap(), 29);
        drop(dir);
    }
}
