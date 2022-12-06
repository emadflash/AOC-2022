#![allow(non_snake_case)]

use std::collections::HashMap;
use std::error::Error;
use std::fs;

fn get_start_of_message(src: &String, window_size: usize) -> u32 {
    assert!(src.len() >= window_size);

    let mut window_hashtable: HashMap<char, u32> = HashMap::new();
    for ch in src[0..window_size].chars() {
        window_hashtable
            .entry(ch)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    for (i, c) in src.chars().skip(window_size).enumerate() {
        if window_hashtable.len() == window_size {
            return (i + window_size) as u32;
        }

        let left_most_char = src.chars().nth(i).unwrap();
        assert!(window_hashtable.contains_key(&left_most_char));

        if window_hashtable[&left_most_char] == 1 {
            window_hashtable.remove(&left_most_char);
        } else {
            *window_hashtable.get_mut(&left_most_char).unwrap() -= 1;
        }

        window_hashtable
            .entry(c)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    // NOTE(madflash) - It should never hit this line but just in case ..
    // Why ?
    // Because we are looking for "start of message" in "message", at the end there is no start
    //
    // End can never be the start of message KEKW
    //                          - Madflash, 2022
    if window_hashtable.len() == window_size {
        return src.len() as u32;
    }

    unreachable!();
}

#[inline]
fn part1(src: &String) -> u32 {
    get_start_of_message(src, 4)
}

#[inline]
fn part2(src: &String) -> u32 {
    get_start_of_message(src, 14)
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
        assert_eq!(5, part1(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()));
        assert_eq!(6, part1(&"nppdvjthqldpwncqszvftbrmjlhg".to_string()));
        assert_eq!(10, part1(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()));
        assert_eq!(11, part1(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(19, part2(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()));
        assert_eq!(23, part2(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()));
        assert_eq!(23, part2(&"nppdvjthqldpwncqszvftbrmjlhg".to_string()));
        assert_eq!(29, part2(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()));
        assert_eq!(26, part2(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()));
    }
}
