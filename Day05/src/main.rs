#![allow(non_snake_case)]

use std::error::Error;
use std::fs;

struct Move {
    from: u32,
    to: u32,
    count: u32,
}

impl Move {
    fn from_line(line: &str) -> Self {
        let chunks = line.split(' ').collect::<Vec<&str>>();
        Self {
            from: chunks.get(3).unwrap().parse::<u32>().unwrap(),
            to: chunks.get(5).unwrap().parse::<u32>().unwrap(),
            count: chunks.get(1).unwrap().parse::<u32>().unwrap(),
        }
    }
}

fn process_input(src: &String) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();

    for line in src.split("\n") {
        let line = line.trim_start();
        if !line.is_empty() {
            match &line[..1] {
                "m" => moves.push(Move::from_line(&line)),
                _ => (),
            }
        }
    }

    moves
}

fn part1(src: &String, crate_stacks: Vec<Vec<char>>) -> String {
    let moves = process_input(src);
    let mut crate_stacks = crate_stacks;

    for m in moves {
        let from = crate_stacks.get_mut(m.from as usize - 1).unwrap();
        let mut crates_to_move = from
            .drain(from.len() - m.count as usize..)
            .rev()
            .collect::<Vec<char>>();

        crate_stacks
            .get_mut(m.to as usize - 1)
            .unwrap()
            .append(&mut crates_to_move);
    }

    crate_stacks
        .into_iter()
        .map(|c| -> char {
            if c.is_empty() {
                return ' ';
            }
            c.last().unwrap().clone()
        })
        .collect::<String>()
}

fn part2(src: &String, crate_stacks: Vec<Vec<char>>) -> String {
    let moves = process_input(src);
    let mut crate_stacks = crate_stacks;

    for m in moves {
        let from = crate_stacks.get_mut(m.from as usize - 1).unwrap();
        let mut crates_to_move = from
            .drain(from.len() - m.count as usize..)
            .collect::<Vec<char>>();

        crate_stacks
            .get_mut(m.to as usize - 1)
            .unwrap()
            .append(&mut crates_to_move);
    }

    crate_stacks
        .into_iter()
        .map(|c| -> char {
            if c.is_empty() {
                return ' ';
            }
            c.last().unwrap().clone()
        })
        .collect::<String>()
}

fn main() -> Result<(), Box<dyn Error>> {
    let src = fs::read_to_string("input.txt")?;
    let crate_stacks: Vec<Vec<char>> = vec![
        vec!['F', 'C', 'J', 'P', 'H', 'T', 'W'],
        vec!['G', 'R', 'V', 'F', 'Z', 'J', 'B', 'H'],
        vec!['H', 'P', 'T', 'R'],
        vec!['Z', 'S', 'N', 'P', 'H', 'T'],
        vec!['N', 'V', 'F', 'Z', 'H', 'J', 'C', 'D'],
        vec!['P', 'M', 'G', 'F', 'W', 'D', 'Z'],
        vec!['M', 'V', 'Z', 'W', 'S', 'J', 'D', 'P'],
        vec!['N', 'D', 'S'],
        vec!['D', 'Z', 'S', 'F', 'M'],
    ];

    println!("{}", part1(&src, crate_stacks.clone()));
    println!("{}", part2(&src, crate_stacks.clone()));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let src = "    [D]    
        [N] [C]    
        [Z] [M] [P]
         1   2   3 

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2";

        let crate_stacks: Vec<Vec<char>> = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!("CMZ", part1(&src.to_string(), crate_stacks));
    }

    #[test]
    fn test_part2() {
        let src = "    [D]    
        [N] [C]    
        [Z] [M] [P]
         1   2   3 

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2";

        let crate_stacks: Vec<Vec<char>> = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!("MCD", part2(&src.to_string(), crate_stacks));
    }
}
