use itertools::Itertools;
use std::ops::Range;

fn parse_input(input: &str) -> impl Iterator<Item = Range<i64>> {
    input.trim().split(",").map(|pair| {
        let (a, b) = pair.split_once("-").unwrap();
        a.parse::<i64>().unwrap()..(b.parse::<i64>().unwrap() + 1)
    })
}

pub fn part_1(input: &str) -> String {
    parse_input(input)
        .flatten()
        .filter(|id| {
            let s = id.to_string();
            s[..s.len() / 2] == s[s.len() / 2..]
        })
        .sum::<i64>()
        .to_string()
}

pub fn part_2(input: &str) -> String {
    parse_input(input)
        .flatten()
        .filter(|id| {
            let s = id.to_string();

            (2..(s.len() + 1)).filter(|d| s.len() % d == 0).any(|d| {
                let step = s.len() / d;
                let start_indexes = (0..(s.len() + 1)).step_by(step);
                let end_indexes = (step..(s.len() + 1)).step_by(step);
                start_indexes
                    .zip(end_indexes)
                    .map(|(a, b)| &s[a..b])
                    .all_equal()
            })
        })
        .sum::<i64>()
        .to_string()
}
