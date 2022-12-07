#![allow(non_snake_case)]

use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

const INPUT: &str = include_str!("../input.txt");

enum Command<'a> {
    Ls,
    Cd(&'a str),
}

impl<'a> Command<'a> {
    pub fn from_shell_input(input: &'a str) -> Self {
        let cmds = input.split(' ').collect::<Vec<_>>();
        if cmds.len() == 2 {
            return Command::Ls;
        }
        return Command::Cd(cmds.last().unwrap());
    }
}

macro_rules! finish_dir_visit {
    ($dir_sizes:ident, $current_path:ident) => {
        let child_size = $dir_sizes[&$current_path.to_string_lossy().to_string()];
        $current_path.pop();
        $dir_sizes
            .entry($current_path.to_string_lossy().to_string())
            .and_modify(|current_size| *current_size += child_size);
    };
}

fn get_dir_sizes(input: &str) -> HashMap<String, u32> {
    let mut dir_sizes: HashMap<String, u32> = HashMap::new();
    let mut current_path = PathBuf::new();

    for line in input.split('\n') {
        if line.is_empty() {
            continue;
        }

        let line = line.trim_start();
        match line.chars().next() {
            Some('$') => {
                let cmd = Command::from_shell_input(line);
                match cmd {
                    Command::Ls => (),
                    Command::Cd(dir) => {
                        if dir == ".." {
                            finish_dir_visit!(dir_sizes, current_path);
                        } else {
                            current_path.push(dir);
                            dir_sizes.insert(current_path.to_string_lossy().to_string(), 0);
                        }
                    }
                };
            }

            Some('0'..='9') => {
                let file_size = line[0..line.find(' ').unwrap()].parse::<u32>().unwrap();
                dir_sizes
                    .entry(current_path.to_string_lossy().to_string())
                    .and_modify(|current_size| *current_size += file_size);
            }

            _ => (),
        }
    }

    while current_path.as_os_str() != "/" {
        finish_dir_visit!(dir_sizes, current_path);
    }

    dir_sizes
}

fn part1(input: &str) -> u32 {
    get_dir_sizes(input)
        .values()
        .cloned()
        .collect::<Vec<u32>>()
        .into_iter()
        .filter(|size| size <= &100000)
        .sum()
}

fn part2(input: &str) -> u32 {
    let dir_sizes = get_dir_sizes(input);
    let unsed_space = 70000000 - dir_sizes["/"];

    assert!(unsed_space <= 30000000);
    let required_space = 30000000 - unsed_space;

    let mut s = dir_sizes.values().cloned().collect::<Vec<u32>>();
    s.sort();

    for size in s.iter() {
        if size >= &required_space {
            return *size;
        }
    }

    unreachable!();
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("{}", part1(INPUT));
    println!("{}", part2(INPUT));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const INPUT: &str = "$ cd /
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
        7214296 k";

    #[test]
    fn test_part1() {
        assert_eq!(95437, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(24933642, part2(INPUT));
    }
}
