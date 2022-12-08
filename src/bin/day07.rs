use advent_of_code_2022::stream_items_from_file;
use std::{collections::HashMap, num::ParseIntError, path::Path, str::FromStr};
use thiserror::Error;

use anyhow::Result;

const INPUT: &str = "input/day07.txt";

#[derive(Debug, Clone)]
enum Command {
    Cd(String),
    Ls,
}

#[derive(Debug, Error)]
enum CommandParseError {
    #[error("Unknown command '{0}'")]
    UnknownCommand(String),
    #[error("Missing command argument (input: '{0}')")]
    MissingArgument(String),
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("ls") {
            Ok(Command::Ls)
        } else if s.starts_with("cd") {
            s.split_once(" ")
                .ok_or_else(|| CommandParseError::MissingArgument(s.to_string()))
                .map(|(_, arg)| Command::Cd(arg.to_string()))
        } else {
            Err(CommandParseError::UnknownCommand(s.to_string()))
        }
    }
}

#[derive(Debug, Clone)]
enum ListingEntry {
    Directory(String),
    File(usize, String),
}

#[derive(Debug, Error)]
enum ListingEntryParseError {
    #[error("Input was not a listing tuple: '{0}'")]
    NotAListingTuple(String),
    #[error("Input prefix unexpected")]
    InvalidPrefix(#[from] ParseIntError),
}

impl FromStr for ListingEntry {
    type Err = ListingEntryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_once(" ")
            .ok_or_else(|| ListingEntryParseError::NotAListingTuple(s.to_string()))
            .and_then(|(prefix, name)| {
                if prefix == "dir" {
                    Ok(ListingEntry::Directory(name.to_string()))
                } else {
                    let size = prefix.parse::<usize>()?;
                    Ok(ListingEntry::File(size, name.to_string()))
                }
            })
    }
}

#[derive(Debug, Clone)]
enum CommandOrListing {
    Command(Command),
    Listing(ListingEntry),
}

#[derive(Debug, Error)]
enum InputParseError {
    #[error("Could not parse command")]
    CommandParseError(#[from] CommandParseError),
    #[error("Listing entry has invalid format")]
    ListingParseError(#[from] ListingEntryParseError),
}

impl FromStr for CommandOrListing {
    type Err = InputParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('$') {
            Ok(CommandOrListing::Command(s[2..].parse()?))
        } else {
            Ok(CommandOrListing::Listing(s.parse()?))
        }
    }
}

#[derive(Debug, Clone)]
enum FileSystemNode {
    Directory(HashMap<String, FileSystemNode>),
    File(usize),
}

impl FileSystemNode {
    fn resolve_mut(&mut self, path: &[String]) -> Option<&mut FileSystemNode> {
        if path.len() == 0 {
            Some(self)
        } else {
            match self {
                FileSystemNode::Directory(children) => children
                    .get_mut(&path[0])
                    .and_then(|child| child.resolve_mut(&path[1..])),
                FileSystemNode::File(_) => None,
            }
        }
    }

    fn add_child_directory(&mut self, name: String) {
        match self {
            FileSystemNode::Directory(children) => {
                children.insert(name, Self::Directory(Default::default()))
            }
            FileSystemNode::File(_) => panic!("Can't add children to a file"),
        };
    }

    fn add_file(&mut self, name: String, size: usize) {
        match self {
            FileSystemNode::Directory(children) => children.insert(name, Self::File(size)),
            FileSystemNode::File(_) => panic!("Can't add children to a file"),
        };
    }

    fn get_size(&self) -> usize {
        match self {
            FileSystemNode::Directory(children) => children.values().map(|c| c.get_size()).sum(),
            FileSystemNode::File(size) => *size,
        }
    }

    // TODO: This could be easily improved by also taking a fold function instead of the
    // vec collection
    fn find_elements<'a, F>(&'a self, pred: &F, target: &mut Vec<&'a Self>)
    where
        F: Fn(&Self) -> bool,
    {
        if pred(self) {
            target.push(self)
        }
        match self {
            FileSystemNode::Directory(children) => {
                children
                    .values()
                    .for_each(|c| c.find_elements(pred, target));
            }
            FileSystemNode::File(_) => (),
        }
    }

    fn is_dir(&self) -> bool {
        match self {
            FileSystemNode::Directory(_) => true,
            FileSystemNode::File(_) => false,
        }
    }
}

fn observe_commands(input: impl Iterator<Item = CommandOrListing>) -> FileSystemNode {
    let mut current_directory = Vec::<String>::new();
    let mut filesystem_root = FileSystemNode::Directory(Default::default());

    for command_or_listing in input {
        match command_or_listing {
            CommandOrListing::Command(command) => match command {
                Command::Cd(path) => {
                    if path == "/" {
                        current_directory.clear();
                    } else if path == ".." {
                        current_directory
                            .pop()
                            .expect("Can not navigate above root");
                    } else {
                        current_directory.push(path);
                    }
                }
                Command::Ls => (),
            },
            CommandOrListing::Listing(listing_entry) => {
                let current_dir = filesystem_root
                    .resolve_mut(&current_directory)
                    .expect("Current path does not exist");
                match listing_entry {
                    ListingEntry::Directory(name) => current_dir.add_child_directory(name),
                    ListingEntry::File(name, size) => current_dir.add_file(size, name),
                }
            }
        }
    }

    filesystem_root
}

fn part1<P: AsRef<Path>>(input: P) -> Result<usize> {
    let fs_state = observe_commands(
        stream_items_from_file::<P, CommandOrListing>(input)?
            .map(|r| r.expect("Invalid line in input")),
    );

    let mut large_dirs = Vec::new();
    fs_state.find_elements(&|e| e.is_dir() && e.get_size() < 100000, &mut large_dirs);

    Ok(large_dirs.iter().map(|d| d.get_size()).sum())
}

fn part2<P: AsRef<Path>>(input: P) -> Result<usize> {
    let fs_state = observe_commands(
        stream_items_from_file::<P, CommandOrListing>(input)?
            .map(|r| r.expect("Invalid line in input")),
    );

    let current_used_space = fs_state.get_size();
    const TOTAL_AVAILABLE: usize = 70000000;
    const REQUIRED: usize = 30000000;
    let current_free = TOTAL_AVAILABLE - current_used_space;
    let need_to_free_up = REQUIRED - current_free;

    let mut could_delete = Vec::new();
    fs_state.find_elements(
        &|e| e.is_dir() && e.get_size() >= need_to_free_up,
        &mut could_delete,
    );

    Ok(could_delete
        .iter()
        .map(|d| d.get_size())
        .min()
        .expect("No suitable directory found"))
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
            indoc![
                "
            $ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            $ cd a
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd e
            $ ls
            584 i
            $ cd ..
            $ cd ..
            $ cd d
            $ ls
            4060174 j
            8033020 d.log
            5626152 d.ext
            7214296 k
        "
            ],
            None,
        );
        assert_eq!(part1(&file).unwrap(), 95437);
        //assert_eq!(part2(&file).unwrap(), 19);
        drop(dir);
    }
}
