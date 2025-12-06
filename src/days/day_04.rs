use itertools::*;

use crate::util::*;

pub fn part_1(input: &str) -> String {
    let grid = add_grid_border(&parse_char_grid(input), '.');
    let n = grid.len();
    let m = grid[0].len();

    let mut accessable_rolls = 0;
    for (i, j) in iproduct!(1..n - 1, 1..m - 1) {
        if grid[i][j] == '.' {
            continue;
        }

        let surrounding_rolls = iproduct!(i - 1..=i + 1, j - 1..=j + 1)
            .filter(|&(i, j)| grid[i][j] == '@')
            .count();

        if surrounding_rolls < 5 {
            accessable_rolls += 1
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

            let surrounding_rolls = iproduct!(i - 1..=i + 1, j - 1..=j + 1)
                .filter(|&(i, j)| grid[i][j] == '@')
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
