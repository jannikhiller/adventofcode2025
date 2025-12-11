use itertools::*;

use crate::util::ParseStr;

type I = u64;

pub fn part_1(input: &str) -> String {
    input
        .parse_points::<I, 2>()
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| {
            izip!(p1, p2)
                .map(|(x1, x2)| x1.abs_diff(*x2) + 1)
                .product::<I>()
        })
        .max()
        .unwrap()
        .to_string()
}

#[allow(unused_variables)]
pub fn part_2(input: &str) -> String {
    String::new()
}
