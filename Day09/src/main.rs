#![allow(non_snake_case)]

use std::collections::HashSet;
use std::error::Error;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Rope {
    x: i32,
    y: i32,
}

impl Rope {
    #[inline]
    fn on_same_row_as(&self, other: &Rope) -> bool {
        self.y == other.y
    }

    #[inline]
    fn on_same_col_as(&self, other: &Rope) -> bool {
        self.x == other.x
    }
}

fn part1(input: &str) -> u32 {
    let mut visited_pos: HashSet<Rope> = HashSet::new();
    let mut tail = Rope { x: 0, y: 0 };
    let mut head = Rope { x: 0, y: 0 };

    for line in input.split("\n").filter(|l| !l.is_empty()) {
        let (dir, count) = line.trim_start().split_once(' ').unwrap();
        let dir = dir.chars().next().unwrap();
        let count = count.parse::<u8>().unwrap();

        // the rope must be quite short; in fact, the head (H) and tail (T) must always be touching
        // (diagonally adjacent and even overlapping both count as touching):
        //
        // 1) If the head is ever two steps directly up, down, left, or right from the tail,
        // the tail must also move one step in that direction so it remains close enough
        //
        // ...    ...    ...
        // .T.    .T.    ...
        // .H. -> ... -> .T.
        // ...    .H.    .H.
        // ...    ...    ...
        //
        // 2) if the head and tail aren't touching and aren't in the same row or column,
        // the tail always moves one step diagonally to keep up
        //
        // .....    .....    .....
        // .....    ..H..    ..H..
        // ..H.. -> ..... -> ..T..
        // .T...    .T...    .....
        // .....    .....    .....

        visited_pos.insert(tail.clone());

        match dir {
            'R' => {
                for _ in 0..count {
                    head.x += 1;

                    if tail.on_same_row_as(&head) && (tail.x - head.x).abs() > 1 {
                        tail.x += 1;
                        visited_pos.insert(tail.clone());
                    } else if (tail.x - head.x).abs() > 1 {
                        tail = head.clone();
                        tail.x -= 1;
                        visited_pos.insert(tail.clone());
                    }
                }
            }

            'L' => {
                for _ in 0..count {
                    head.x -= 1;

                    if tail.on_same_row_as(&head) && (tail.x - head.x).abs() > 1 {
                        tail.x -= 1;
                        visited_pos.insert(tail.clone());
                    } else if (tail.x - head.x).abs() > 1 {
                        tail = head.clone();
                        tail.x += 1;
                        visited_pos.insert(tail.clone());
                    }
                }
            }

            'U' => {
                for _ in 0..count {
                    head.y -= 1;

                    if tail.on_same_col_as(&head) && (tail.y - head.y).abs() > 1 {
                        tail.y -= 1;
                        visited_pos.insert(tail.clone());
                    } else if (tail.y - head.y).abs() > 1 {
                        tail = head.clone();
                        tail.y += 1;
                        visited_pos.insert(tail.clone());
                    }
                }
            }

            'D' => {
                for _ in 0..count {
                    head.y += 1;

                    if head.on_same_col_as(&tail) && (tail.y - head.y).abs() > 1 {
                        tail.y += 1;
                        visited_pos.insert(tail.clone());
                    } else if (tail.y - head.y).abs() > 1 {
                        tail = head.clone();
                        tail.y -= 1;
                        visited_pos.insert(tail.clone());
                    }
                }
            }

            _ => (),
        }
    }

    visited_pos.len() as u32
}

fn part2(input: &str) -> u32 {
    todo!();
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const INPUT: &str = "R 4
        U 4
        L 3
        D 1
        R 4
        D 1
        L 5
        R 2";

    const INPUT2: &str = "
        R 5
        U 8
        L 8
        D 3
        R 17
        D 10
        L 25
        U 20";

    #[test]
    fn test_part1() {
        assert_eq!(13, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1, part2(INPUT));
        assert_eq!(36, part2(INPUT2));
    }
}
