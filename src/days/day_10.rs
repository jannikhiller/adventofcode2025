use good_lp::*;
use std::collections::HashSet;

use itertools::*;

type I = u16;

fn parse_part_1(input: &str) -> Vec<(I, Vec<I>)> {
    let parse_line = |line: &str| {
        let mut parts = line.split_whitespace();
        let target_str = parts.next().unwrap();
        parts.next_back();

        let target = target_str
            .trim_matches(['[', ']'])
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '#')
            .fold(0, |acc, (i, _)| acc | (1 << i));

        let buttons = parts
            .map(|button_str| {
                button_str
                    .trim_matches(['(', ')'])
                    .split(",")
                    .fold(0, |acc, i| acc | (1 << i.parse::<I>().unwrap()))
            })
            .collect();
        (target, buttons)
    };
    input.lines().map(parse_line).collect()
}

fn get_min_presses(target: I, buttons: &[I]) -> usize {
    if target == 0 {
        return 0;
    }
    let mut seen = HashSet::from([0]);
    let mut curr_nodes = vec![0];

    for presses in 1.. {
        let mut new_nodes = vec![];
        for config in curr_nodes {
            for button in buttons {
                let resulting_config = config ^ button;
                if resulting_config == target {
                    return presses;
                }
                if !seen.insert(resulting_config) {
                    continue;
                }
                new_nodes.push(resulting_config);
            }
        }
        curr_nodes = new_nodes;
    }
    unreachable!()
}

pub fn part_1(input: &str) -> String {
    parse_part_1(input)
        .into_iter()
        .map(|(target, buttons)| get_min_presses(target, &buttons))
        .sum::<usize>()
        .to_string()
}

fn get_min_presses_joltage(target: &[I], buttons: &[Vec<I>]) -> usize {
    let num_vars = buttons.len();
    variables! {vars: 0 <= x[num_vars] (integer);}

    let objective = x.iter().sum::<Expression>();
    let constraints = target.iter().enumerate().map(|(i, value)| {
        izip!(&x, buttons)
            .map(|(x_b, b)| {
                if b.contains(&(i as I)) {
                    x_b.into_expression()
                } else {
                    0.into_expression()
                }
            })
            .sum::<Expression>()
            .eq(value.into_expression())
    });

    let problem = vars
        .minimise(&objective)
        .using(microlp)
        .with_all(constraints);

    problem.solve().unwrap().eval(objective) as usize
}

fn parse_part_2(input: &str) -> Vec<(Vec<I>, Vec<Vec<I>>)> {
    let parse_line = |line: &str| {
        let mut parts = line.split_whitespace();
        parts.next();
        let target_str = parts.next_back().unwrap();

        let target = target_str
            .trim_matches(['{', '}'])
            .split(",")
            .map(|i| i.parse().unwrap())
            .collect();

        let buttons: Vec<Vec<I>> = parts
            .map(|button_str| {
                button_str
                    .trim_matches(['(', ')'])
                    .split(",")
                    .map(|i| i.parse().unwrap())
                    .collect()
            })
            .collect();
        (target, buttons)
    };
    input.lines().map(parse_line).collect()
}

pub fn part_2(input: &str) -> String {
    parse_part_2(input)
        .into_iter()
        .map(|(target, buttons)| get_min_presses_joltage(&target, &buttons))
        .sum::<usize>()
        .to_string()
}
