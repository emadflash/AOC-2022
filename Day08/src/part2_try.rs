fn part2(input: &str) -> u32 {
    let grid = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.trim_start().chars().map(|c| c as u8).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let m = grid.len();
    let n = grid.first().unwrap().len();

    let mut top = vec![1; n];
    let mut scenic_score: Vec<Vec<u32>> = vec![vec![0; n]; m];

    let mut left: u32 = 1;

    for i in 1..n {
        scenic_score[0][i] = left;
        if grid[0][i] >= grid[0][i - 1] {
            left = 1;
        } else {
            left += 1;
        }
    }

    left = 1; // cleanup(madflash) - Rename this
    for i in 1..m {
        scenic_score[i][0] = left;
        if grid[i][0] >= grid[i - 1][0] {
            left = 1;
        } else {
            left += 1;
        }
    }

    left = 1;
    for i in 1..m {
        for j in 1..n {
            println!("left: {}, top: {}", left, top[i - 1]);
            scenic_score[i][j] = top[j - 1] * left;

            if grid[i][j] >= grid[i - 1][j] {
                top[j - 1] = 1;
            } else {
                top[j - 1] += 1;
            }

            if grid[i][j] >= grid[i][j - 1] {
                left = 1;
            } else {
                left += 1;
            }
        }
        left = 1;
    }

    top = vec![1; n];
    let mut highest_scenic_score: u32 = u32::MIN;

    left = 1;
    for i in 1..n {
        scenic_score[0][i] = left;
        if grid[0][i] >= grid[0][i - 1] {
            left = 1;
        } else {
            left += 1;
        }
    }

    left = 1; // cleanup(madflash) - Rename this
    for i in 1..m {
        scenic_score[i][0] = left;
        if grid[i][0] >= grid[i - 1][0] {
            left = 1;
        } else {
            left += 1;
        }
    }

    left = 1;
    for i in (0..m - 1).rev() {
        for j in (0..n - 1).rev() {
            scenic_score[i][j] *= top[j] * left;
            highest_scenic_score = std::cmp::max(highest_scenic_score, scenic_score[i][j]);

            if grid[i][j] >= grid[i + 1][j] {
                top[j] = 1;
            } else {
                top[j] += 1;
            }

            if grid[i][j] >= grid[i][j + 1] {
                left = 1;
            } else {
                left += 1;
            }
        }

        left = 1;
    }

    println!("{:?}", scenic_score);
    highest_scenic_score
}
