#![allow(non_snake_case)]

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input_file() -> Result<Vec<[char; 2]>, Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut moves: Vec<[char; 2]> = Vec::new();

    for line in reader.lines() {
        if let Some((a, x)) = line?.split_once(' ') {
            moves.push([a.chars().nth(0).unwrap(), x.chars().nth(0).unwrap()]);
        } else {
            unreachable!();
        }
    }

    Ok(moves)
}

fn part1(moves: &Vec<[char; 2]>) -> u32 {
    // Points for game -
    // 0 - los
    // 3 - draw
    // 6 - when you win

    // Points for choosing -
    // 1 - Rock
    // 2 - Paper
    // 3 - Scissor
    let lookup_move: HashMap<&str, u8> = [
        ("AX", 4),
        ("AY", 8),
        ("AZ", 3),
        ("BX", 1),
        ("BY", 5),
        ("BZ", 9),
        ("CX", 7),
        ("CY", 2),
        ("CZ", 6),
    ]
    .iter()
    .cloned()
    .collect();

    moves
        .iter()
        .map(|a| -> u32 { lookup_move[a.iter().collect::<String>().as_str()].into() })
        .sum()
}

fn part2(moves: &Vec<[char; 2]>) -> u32 {
    // Meaning for me (second column) -
    // X - need to lose
    // Y - need to draw
    // Z - need to win

    let lookup_move: HashMap<&str, u8> = [
        ("AX", 3),
        ("AY", 4),
        ("AZ", 8),
        ("BX", 1),
        ("BY", 5),
        ("BZ", 9),
        ("CX", 2),
        ("CY", 6),
        ("CZ", 7),
    ]
    .iter()
    .cloned()
    .collect();

    moves
        .iter()
        .map(|a| -> u32 { lookup_move[a.iter().collect::<String>().as_str()].into() })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let moves = read_input_file()?;
    println!("{}", part1(&moves));
    println!("{}", part2(&moves));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let moves: Vec<[char; 2]> = vec![['A', 'Y'], ['B', 'X'], ['C', 'Z']];
        assert_eq!(15, part1(&moves));
    }

    #[test]
    fn test_part2() {
        let moves: Vec<[char; 2]> = vec![['A', 'Y'], ['B', 'X'], ['C', 'Z']];
        assert_eq!(12, part2(&moves));
    }
}
