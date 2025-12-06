use itertools::*;

use crate::util::*;

pub fn part_1(input: &str) -> String {
    let grid = add_grid_border(&parse_char_grid(input), '.');
    let n = grid.len();
    let m = grid[0].len();

    let mut accessable_rolls = 0;
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            if grid[i][j] == '.' {
                continue;
            }

            let surrounding_rolls = [-1i32, 0, 1]
                .iter()
                .cartesian_product([-1i32, 0, 1].iter())
                .filter(|&(di, dj)| grid[(i as i32 + di) as usize][(j as i32 + dj) as usize] == '@')
                .count();

            if surrounding_rolls < 5 {
                accessable_rolls += 1
            }
        }
    }

    accessable_rolls.to_string()
}

pub fn part_2(input: &str) -> String {
    let mut grid = add_grid_border(&parse_char_grid(input), '.');
    let n = grid.len();
    let m = grid[0].len();

    let mut accessable_rolls = 0;
    let mut changed = true;

    while changed {
        changed = false;
        for (i, j) in iproduct!(1..n - 1, 1..m - 1) {
            if grid[i][j] == '.' {
                continue;
            }

            let surrounding_rolls = iproduct!([-1i32, 0, 1], [-1i32, 0, 1])
                .filter(|&(di, dj)| grid[(i as i32 + di) as usize][(j as i32 + dj) as usize] == '@')
                .count();

            if surrounding_rolls < 5 {
                accessable_rolls += 1;
                grid[i][j] = '.';
                changed = true;
            }
        }
    }

    accessable_rolls.to_string()
}
