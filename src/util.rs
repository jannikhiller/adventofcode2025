use std::fmt::{Debug, Display};
use std::str::FromStr;

use itertools::Itertools;

pub trait Grid<T> {
    #[allow(dead_code)]
    fn print(&self)
    where
        T: Display;
    fn add_border(&self, filler: T) -> Vec<Vec<T>>
    where
        T: Copy;
}

impl<T> Grid<T> for [Vec<T>] {
    fn print(&self)
    where
        T: Display,
    {
        print_grid(self);
    }
    fn add_border(&self, filler: T) -> Vec<Vec<T>>
    where
        T: Copy,
    {
        add_grid_border(self, filler)
    }
}

pub trait Parseable = FromStr + Clone where <Self as FromStr>::Err: Debug;
pub trait SplitFn<'a, I: Iterator<Item = &'a str>> = Fn(&'a str) -> I;

#[allow(dead_code)]
pub trait ParseStr {
    fn parse_char_grid(&self) -> Vec<Vec<char>>;
    fn parse_grid_rows<T: Parseable>(&self) -> Vec<Vec<T>>;
    fn parse_grid_rows_with_splitter<'a, T: Parseable, F: SplitFn<'a, I>, I>(
        &'a self,
        splitter: F,
    ) -> Vec<Vec<T>>;

    fn parse_grid_cols<T: Parseable>(&self) -> Vec<Vec<T>>;
    fn parse_grid_cols_with_splitter<'a, T: Parseable, F: SplitFn<'a, I>, I>(
        &'a self,
        splitter: F,
    ) -> Vec<Vec<T>>;
    fn parse_points<T: Parseable, const N: usize>(&self) -> Vec<[T; N]>;
}

#[allow(dead_code)]
impl ParseStr for str {
    fn parse_char_grid(&self) -> Vec<Vec<char>> {
        parse_char_grid(self)
    }
    fn parse_grid_rows<T: Parseable>(&self) -> Vec<Vec<T>> {
        parse_grid_rows(self)
    }
    fn parse_grid_rows_with_splitter<'a, T: Parseable, F: SplitFn<'a, I>, I>(
        &'a self,
        splitter: F,
    ) -> Vec<Vec<T>> {
        parse_grid_rows_with_splitter(self, splitter)
    }
    fn parse_grid_cols<T: Parseable>(&self) -> Vec<Vec<T>> {
        parse_grid_cols(self)
    }
    fn parse_grid_cols_with_splitter<'a, T: Parseable, F: SplitFn<'a, I>, I>(
        &'a self,
        splitter: F,
    ) -> Vec<Vec<T>> {
        parse_grid_cols_with_splitter(self, splitter)
    }
    fn parse_points<T: Parseable, const N: usize>(&self) -> Vec<[T; N]> {
        parse_points(self)
    }
}

pub fn add_grid_border<T: Copy>(grid: &[Vec<T>], filler: T) -> Vec<Vec<T>> {
    let m = grid[0].len();

    let mut grid_with_border = vec![vec![filler; m + 2]];
    for row in grid {
        let mut bordered_row = vec![filler];
        bordered_row.extend(row);
        bordered_row.push(filler);
        grid_with_border.push(bordered_row);
    }
    grid_with_border.push(vec![filler; m + 2]);

    grid_with_border
}

#[allow(dead_code)]
pub fn print_grid<T: Display>(grid: &[Vec<T>]) {
    for r in grid {
        for c in r {
            print!("{c}");
        }
        println!();
    }
}

pub fn parse_char_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

pub fn parse_points<const N: usize, T: Parseable>(input: &str) -> Vec<[T; N]> {
    input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|x| x.parse().unwrap())
                .collect_array()
                .unwrap()
        })
        .collect()
}

#[allow(dead_code)]
pub fn parse_grid_rows<T>(input: &str) -> Vec<Vec<T>>
where
    T: FromStr + Clone,
    <T as FromStr>::Err: Debug,
{
    parse_grid_rows_with_splitter(input, |s| s.split("").filter(|s| !s.is_empty()))
}

pub fn parse_grid_rows_with_splitter<'a, T: Parseable, F: SplitFn<'a, I>, I>(
    input: &'a str,
    splitter: F,
) -> Vec<Vec<T>> {
    input
        .lines()
        .map(|line| {
            splitter(line)
                .filter(|s| !s.is_empty())
                .map(|x| x.parse().unwrap())
                .collect()
        })
        .collect()
}

#[allow(dead_code)]
pub fn parse_grid_cols<T: Parseable>(input: &str) -> Vec<Vec<T>> {
    parse_grid_cols_with_splitter(input, |s| s.split("").filter(|s| !s.is_empty()))
}

pub fn parse_grid_cols_with_splitter<'a, T: Parseable, F: SplitFn<'a, I>, I>(
    input: &'a str,
    splitter: F,
) -> Vec<Vec<T>> {
    let mut grid = vec![];
    let mut first_line = true;
    for line in input.lines() {
        let elems: Vec<T> = splitter(line).map(|x| x.parse().unwrap()).collect();
        if first_line {
            grid = vec![vec![]; elems.len()];
            first_line = false;
        }
        for (i, elem) in elems.into_iter().enumerate() {
            grid[i].push(elem);
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_grid_rows() {
        let input = "1 2 3\n4 5 6\n7 8 9";
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let result: Vec<Vec<i64>> = parse_grid_rows_with_splitter(input, |s| s.split_whitespace());
        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_grid_cols() {
        let input = "1 2 3\n4 5 6\n7 8 9";
        let expected = vec![vec![1i64, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
        let result: Vec<Vec<i64>> = parse_grid_cols_with_splitter(input, |s| s.split_whitespace());
        assert_eq!(expected, result);
    }
}
