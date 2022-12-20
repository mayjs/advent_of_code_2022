use advent_of_code_2022::stream_items_from_file;
use anyhow::anyhow;
use anyhow::Result;
use std::ops::Add;
use std::{collections::HashSet, path::Path, str::FromStr};

const INPUT: &str = "input/day18.txt";

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct VoxelCoordinate {
    x: isize,
    y: isize,
    z: isize,
}

impl VoxelCoordinate {
    fn new(x: isize, y: isize, z: isize) -> Self {
        VoxelCoordinate { x, y, z }
    }
}

impl FromStr for VoxelCoordinate {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s
            .split(',')
            .map(|v| v.parse())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| anyhow!("Could not parse number"))?;

        if coords.len() != 3 {
            return Err(anyhow!("Not enough coordinates"));
        }

        Ok(VoxelCoordinate {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        })
    }
}

impl Add for VoxelCoordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        VoxelCoordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let voxels = stream_items_from_file::<_, VoxelCoordinate>(input)?
        .map(|mv| mv.unwrap())
        .collect::<HashSet<_>>();
    let neighbor_deltas = [
        VoxelCoordinate::new(1, 0, 0),
        VoxelCoordinate::new(-1, 0, 0),
        VoxelCoordinate::new(0, 1, 0),
        VoxelCoordinate::new(0, -1, 0),
        VoxelCoordinate::new(0, 0, 1),
        VoxelCoordinate::new(0, 0, -1),
    ];

    Ok(voxels
        .iter()
        .map(|v| {
            neighbor_deltas
                .iter()
                .map(|delta| *v + *delta)
                .filter(|neighbor| !voxels.contains(neighbor))
                .count()
        })
        .sum())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let voxels = stream_items_from_file::<_, VoxelCoordinate>(input)?
        .map(|mv| mv.unwrap())
        .collect::<HashSet<_>>();
    let neighbor_deltas = [
        VoxelCoordinate::new(1, 0, 0),
        VoxelCoordinate::new(-1, 0, 0),
        VoxelCoordinate::new(0, 1, 0),
        VoxelCoordinate::new(0, -1, 0),
        VoxelCoordinate::new(0, 0, 1),
        VoxelCoordinate::new(0, 0, -1),
    ];

    // Calculate two layers of air from each surface, this will help us work around the diagonal
    // movement restriction without hassle
    let first_air_layer = voxels
        .iter()
        .flat_map(|v| {
            neighbor_deltas
                .iter()
                .map(|delta| *v + *delta)
                .filter(|neighbor| !voxels.contains(neighbor))
                .collect::<HashSet<_>>()
        })
        .collect::<HashSet<_>>();
    let second_air_layer = first_air_layer
        .iter()
        .flat_map(|v| {
            neighbor_deltas
                .iter()
                .map(|delta| *v + *delta)
                .filter(|neighbor| !voxels.contains(neighbor))
                .collect::<HashSet<_>>()
        })
        .collect::<HashSet<_>>();

    // Every air voxel we found before is a potential surface air voxel for now
    let mut potential_outside_air_voxels = first_air_layer
        .union(&second_air_layer)
        .collect::<HashSet<_>>();
    // This wil contain the actual first layer outside air voxels in the end
    let mut outside_air_voxels = HashSet::new();
    // Initiate a BFS from the largest air voxel we know of to make sure that we start from the
    // outside
    let mut remaining = vec![**potential_outside_air_voxels.iter().max().unwrap()];

    while let Some(current) = remaining.pop() {
        // This voxel will be definitely classified in this iteration, so remove it from the
        // potential set
        potential_outside_air_voxels.remove(&current);
        if first_air_layer.contains(&current) {
            // This air voxel is part of the first layer outside air voxels
            outside_air_voxels.insert(current);
        }
        // Look for neighbors of this voxel that are still candidates
        neighbor_deltas
            .iter()
            .map(|delta| current + *delta)
            .filter(|neighbor| potential_outside_air_voxels.contains(neighbor))
            .for_each(|n| remaining.push(n));
    }

    Ok(voxels
        .iter()
        .map(|v| {
            neighbor_deltas
                .iter()
                .map(|delta| *v + *delta)
                .filter(|neighbor| outside_air_voxels.contains(neighbor))
                .count()
        })
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
    fn test_d14_examples() {
        let (dir, file) = create_example_file(
            indoc! {"
                2,2,2
                1,2,2
                3,2,2
                2,1,2
                2,3,2
                2,2,1
                2,2,3
                2,2,4
                2,2,6
                1,2,5
                3,2,5
                2,1,5
                2,3,5
            "},
            None,
        );
        assert_eq!(part1(&file).unwrap(), 64);
        assert_eq!(part2(&file).unwrap(), 58);
        drop(dir);
    }
}
