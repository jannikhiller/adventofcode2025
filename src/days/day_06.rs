use std::iter::once;

use crate::util::*;

pub fn part_1(input: &str) -> String {
    let (nums_str, ops_str) = input.trim().rsplit_once("\n").unwrap();
    let nums: Vec<Vec<i64>> = parse_grid_cols_with_splitter(nums_str, |s| s.split_whitespace());
    let ops = ops_str.split_whitespace().map(|op_str| match op_str {
        "+" => |a, b| a + b,
        "*" => |a, b| a * b,
        _ => panic!(),
    });

    nums.into_iter()
        .zip(ops)
        .map(|(col, op)| col.into_iter().reduce(op).unwrap())
        .sum::<i64>()
        .to_string()
}

pub fn part_2(input: &str) -> String {
    let grid = parse_grid_rows::<char>(input);
    let (nums, sign_row) = grid.split_at(grid.len() - 1);
    let sign_row = sign_row.first().unwrap();

    let mut col_start = 0;
    let mut op_char = '_';
    let mut sum = 0;
    for (i, c) in sign_row.iter().copied().chain(once('_')).enumerate() {
        if c == ' ' {
            continue;
        }

        // do the calculation for the last block of numbers
        let col_to_num = |j: usize| {
            (0..nums.len())
                .map(|i| nums[i][j])
                .collect::<String>()
                .trim()
                .parse::<i64>()
                .unwrap()
        };
        let op = match op_char {
            '+' => |a, b| a + b,
            '*' => |a, b| a * b,
            _ => |_, _| panic!(),
        };
        if i > 0 {
            sum += (col_start..i - 1).map(col_to_num).reduce(op).unwrap();
        }

        // start next block of numbers
        col_start = i;
        op_char = c;
    }

    sum.to_string()
}
