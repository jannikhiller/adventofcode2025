use itertools::Itertools;

use crate::util::*;

pub fn part_1(input: &str) -> String {
    let mut grid = input.parse_char_grid().add_border('.');
    let i = grid[1].iter().position(|&c| c == 'S').unwrap();
    grid[1][i] = '|';
    let mut splits = 0;

    for (i, ni) in (0..grid.len()).tuple_windows() {
        for j in 0..grid[0].len() {
            match (grid[i][j], grid[ni][j]) {
                ('|', '.') => grid[ni][j] = '|',
                ('|', '^') => {
                    grid[ni][j - 1] = '|';
                    grid[ni][j + 1] = '|';
                    splits += 1;
                }
                _ => {}
            }
        }
    }

    splits.to_string()
}

pub fn part_2(input: &str) -> String {
    let mut grid = input.parse_char_grid().add_border('.');
    let i = grid[1].iter().position(|&c| c == 'S').unwrap();
    grid[1][i] = '|';

    let mut num_paths = vec![0; grid[0].len()];
    num_paths[i] = 1;

    for (i, ni) in (0..grid.len()).tuple_windows() {
        for j in 0..grid[0].len() {
            match (grid[i][j], grid[ni][j]) {
                ('|', '.') => grid[ni][j] = '|',
                ('|', '^') => {
                    grid[ni][j - 1] = '|';
                    grid[ni][j + 1] = '|';
                    num_paths[j - 1] += num_paths[j];
                    num_paths[j + 1] += num_paths[j];
                    num_paths[j] = 0;
                }
                _ => {}
            }
        }
    }

    num_paths.into_iter().sum::<i64>().to_string()
}
