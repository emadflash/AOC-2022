#![allow(non_snake_case)]

use std::error::Error;

const INPUT: &str = include_str!("../input.txt");

fn part1(input: &str) -> u32 {
    let grid = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_start().chars().map(|c| c as u8).collect::<Vec<_>>())
        .collect::<Vec<Vec<u8>>>();

    let m = grid.len();
    let n = grid.first().unwrap().len();
    let mut is_visible = vec![vec![false; n]; m];
    let mut visible_tree_count: u32 = 0;

    //
    // Visible from top and left       Visible from bottom and right
    // ....^....                        .........
    // ....\....                        .........
    // ....\....                        .........
    // <---t....                        ....t--->
    // .........                        ....\....
    // .........                        ....\....
    // .........                        ....V....
    // .........                        .........
    // .........                        .........
    //

    let mut top_trees_max = grid[0][1..n - 1].iter().cloned().collect::<Vec<u8>>();
    let mut left_tree_max: u8;

    for i in 1..m - 1 {
        left_tree_max = grid[i][0];

        for j in 1..n - 1 {
            if grid[i][j] > top_trees_max[j - 1] || grid[i][j] > left_tree_max {
                is_visible[i][j] = true;
                visible_tree_count += 1;

                if grid[i][j] > top_trees_max[j - 1] {
                    top_trees_max[j - 1] = grid[i][j];
                }

                if grid[i][j] > left_tree_max {
                    left_tree_max = grid[i][j];
                }
            }
        }
    }

    let mut bottom_trees_max = grid[m - 1][1..n - 1].iter().cloned().collect::<Vec<u8>>();
    let mut right_tree_max: u8;

    for i in (1..m - 1).rev() {
        right_tree_max = grid[i][n - 1];

        for j in (1..n - 1).rev() {
            if grid[i][j] > bottom_trees_max[j - 1] || grid[i][j] > right_tree_max {
                if !is_visible[i][j] {
                    visible_tree_count += 1;
                }

                if grid[i][j] > bottom_trees_max[j - 1] {
                    bottom_trees_max[j - 1] = grid[i][j];
                }

                if grid[i][j] > right_tree_max {
                    right_tree_max = grid[i][j];
                }
            }
        }
    }

    visible_tree_count + m as u32 * 2 + (n - 2) as u32 * 2
}

// cleanup(madflash) - make this work in O(n^2)
fn part2(input: &str) -> u32 {
    let grid = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_start().chars().map(|c| c as u8).collect::<Vec<_>>())
        .collect::<Vec<Vec<u8>>>();

    let m = grid.len();
    let n = grid.first().unwrap().len();
    let mut highest_scenic_score: u32 = u32::MIN;

    for i in 0..m {
        for j in 0..n {
            let mut r = 0;
            let mut l = 0;
            let mut t = 0;
            let mut b = 0;

            // Right
            for k in j + 1..n {
                if grid[i][j] <= grid[i][k] {
                    r += 1;
                    break;
                }
                r += 1;
            }

            // Left
            for k in (0..j).rev() {
                if grid[i][j] <= grid[i][k] {
                    l += 1;
                    break;
                }
                l += 1;
            }

            // Top
            for k in (0..i).rev() {
                if grid[i][j] <= grid[k][j] {
                    t += 1;
                    break;
                }
                t += 1;
            }

            // Bottom
            for k in i + 1..m {
                if grid[i][j] <= grid[k][j] {
                    b += 1;
                    break;
                }
                b += 1;
            }

            highest_scenic_score = std::cmp::max(highest_scenic_score, r * l * t * b);
        }
    }

    highest_scenic_score
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("part 1: {}", part1(INPUT));
    println!("part 2: {}", part2(INPUT));

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};
    const INPUT: &str = "30373
        25512
        65332
        33549
        35390";

    #[test]
    fn test_part1() {
        assert_eq!(21, part1(INPUT));
    }

    #[test]
    fn test_part2() {
        assert_eq!(8, part2(INPUT));
    }
}
