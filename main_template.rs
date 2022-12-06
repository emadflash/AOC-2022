#![allow(non_snake_case)]

use std::error::Error;
use std::fs;

fn part1(src: &String) -> u32 {
    todo!(".....ATTEMPT PART 1 HERE....");
}

fn part2(src: &String) -> u32 {
    todo!("....ATTEMPT PART 2 HERE....");
}

fn main() -> Result<(), Box<dyn Error>> {
    let src = fs::read_to_string("input.txt")?;

    println!("{}", part1(&src));
    println!("{}", part2(&src));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        let src = "....TEST STRING FOR PART 1....".to_string();
        assert_eq!(1337, part1(&src));
    }

    #[test]
    fn test_part2() {
        let src = "....TEST STRING FOR PART 2....".to_string();
        assert_eq!(1337, part2(&src));
    }
}
