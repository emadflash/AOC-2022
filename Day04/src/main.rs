#![allow(non_snake_case)]

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input_file() -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut assignment_pairs: Vec<String> = Vec::new();

    for line in reader.lines() {
        assignment_pairs.push(line?.trim().to_string());
    }

    Ok(assignment_pairs)
}

struct Floor {
    lo: u8,
    hi: u8,
}

impl Floor {
    fn from_range_str(range_str: &str) -> Self {
        let (lo, hi) = range_str.split_once('-').unwrap();
        Self {
            lo: lo.parse::<u8>().unwrap(),
            hi: hi.parse::<u8>().unwrap(),
        }
    }
}

fn get_pairs(assignment_pairs: &Vec<String>) -> Vec<(Floor, Floor)> {
    assignment_pairs
        .into_iter()
        .map(|p| -> (Floor, Floor) {
            let (a, b) = p.split_once(',').unwrap();
            (Floor::from_range_str(a), Floor::from_range_str(b))
        })
        .collect()
}

fn part1(assignment_pairs: &Vec<String>) -> u32 {
    get_pairs(assignment_pairs)
        .into_iter()
        .map(|f| -> u32 {
            ((f.0.lo <= f.1.lo && f.0.hi >= f.1.hi) || (f.1.lo <= f.0.lo && f.1.hi >= f.0.hi))
                as u32
        })
        .sum()
}

fn part2(assignment_pairs: &Vec<String>) -> u32 {
    get_pairs(assignment_pairs)
        .into_iter()
        .map(|f| -> u32 {
            ((f.0.lo <= f.1.lo && f.0.hi >= f.1.lo) || (f.1.lo <= f.0.lo && f.1.hi >= f.0.lo))
                as u32
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let assignment_pairs = read_input_file()?;

    println!("{}", part1(&assignment_pairs));
    println!("{}", part2(&assignment_pairs));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let assignment_pairs: Vec<String> = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
        ];
        assert_eq!(2, part1(&assignment_pairs));
    }

    #[test]
    fn test_part2() {
        let assignment_pairs: Vec<String> = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
        ];
        assert_eq!(4, part2(&assignment_pairs));
    }
}
