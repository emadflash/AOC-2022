#![allow(non_snake_case)]

use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input_file() -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut rucksack_items: Vec<String> = Vec::new();

    for line in reader.lines() {
        rucksack_items.push(line?.trim().to_string());
    }

    Ok(rucksack_items)
}

fn get_priority(item_type: char) -> u8 {
    let diff_with: char;
    let offset;

    if item_type.is_lowercase() {
        diff_with = 'a';
        offset = 1
    } else {
        diff_with = 'A';
        offset = 27;
    }

    (item_type as u8 - diff_with as u8) + offset
}

fn part1(rucksack_items: &Vec<String>) -> u32 {
    let get_matching_item = |item: &String| -> char {
        let (a, b) = item.split_at(item.len() / 2);
        let remember_items: HashSet<char> = a.chars().collect();

        for c in b.chars() {
            if remember_items.contains(&c) {
                return c;
            }
        }

        unreachable!()
    };

    rucksack_items
        .iter()
        .map(|i: &String| -> u32 { get_priority(get_matching_item(i)) as u32 })
        .sum()
}

fn part2(rucksack_items: &Vec<String>) -> u32 {
    let mut rucksack_grps: Vec<Vec<&String>> = Vec::new();
    let mut grp_items: Vec<&String> = Vec::new();

    for i in 0..rucksack_items.len() {
        if i != 0 && i % 3 == 0 {
            rucksack_grps.push(grp_items.clone());
            grp_items.clear();
        }
        grp_items.push(rucksack_items.get(i).as_ref().unwrap());
    }
    rucksack_grps.push(grp_items);

    let get_matching_item = |grp: Vec<&String>| -> char {
        let remember_items: HashSet<char> = grp.get(0).unwrap().chars().collect();
        let c = grp.get(2).unwrap();

        for b in grp.get(1).unwrap().chars() {
            if remember_items.contains(&b) && c.contains(b) {
                return b;
            }
        }

        unreachable!()
    };

    rucksack_grps
        .into_iter()
        .map(|i: Vec<&String>| -> u32 { get_priority(get_matching_item(i)) as u32 })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let rucksack_items = read_input_file()?;

    println!("{}", part1(&rucksack_items));
    println!("{}", part2(&rucksack_items));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let rucksack_items: Vec<String> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];
        assert_eq!(157, part1(&rucksack_items));
    }

    #[test]
    fn test_part2() {
        let rucksack_items: Vec<String> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];
        assert_eq!(70, part2(&rucksack_items));
    }
}
