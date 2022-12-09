#![allow(non_snake_case)]

use std::error::Error;

const INPUT: &str = include_str!("../input.txt");

fn part1(input: &str) -> u32 {
    todo!(".....ATTEMPT PART 1 HERE....");
}

fn part2(input: &str) -> u32 {
    todo!("....ATTEMPT PART 2 HERE....");
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const INPUT: &str = "....TEST STRING....";

    #[test]
    fn test_part1() {
        assert_eq!(1337, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1337, part2(INPUT));
    }
}
