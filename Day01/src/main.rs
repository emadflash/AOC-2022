#![allow(non_snake_case)]

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input_file() -> Result<Vec<Vec<i32>>, Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut calories: Vec<Vec<i32>> = Vec::new();
    let mut elf_calories: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            calories.push(elf_calories.clone());
            elf_calories.clear();
        } else {
            elf_calories.push(line.trim().parse::<i32>()?);
        }
    }

    if !elf_calories.is_empty() {
        calories.push(elf_calories.clone());
    }

    Ok(calories)
}

fn part1(calories: &Vec<Vec<i32>>) -> i32 {
    let calories_sum = calories
        .iter()
        .map(|a| a.iter().sum())
        .collect::<Vec<i32>>();

    *calories_sum.iter().max().expect("maximum calorie!")
}

fn part2(calories: &Vec<Vec<i32>>) -> i32 {
    let mut calories_sum = calories
        .iter()
        .map(|a| a.iter().sum())
        .collect::<Vec<i32>>();

    calories_sum.sort();
    calories_sum.iter().rev().take(3).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let calories = read_input_file()?;
    println!("{}", part1(&calories));
    println!("{}", part2(&calories));
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let calories: Vec<Vec<i32>> = vec![
            vec![1000, 2000, 300],
            vec![400],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];

        assert_eq!(part1(&calories), 24000);
    }

    #[test]
    fn test_part2() {
        let calories: Vec<Vec<i32>> = vec![
            vec![1000, 2000, 300],
            vec![400],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];

        assert_eq!(part2(&calories), 45000);
    }
}
