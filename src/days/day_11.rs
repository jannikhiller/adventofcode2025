use itertools::*;
use std::collections::HashMap;

fn parse_input(input: &str) -> HashMap<&str, Vec<&str>> {
    input
        .lines()
        .map(|line| {
            let (out_node, in_nodes) = line.split(":").collect_tuple().unwrap();
            let in_nodes: Vec<_> = in_nodes.split_whitespace().collect();
            (out_node, in_nodes)
        })
        .collect()
}

fn num_paths<'a>(
    from: &'a str,
    to: &str,
    graph: &HashMap<&str, Vec<&'a str>>,
    memo: &mut HashMap<&'a str, usize>,
) -> usize {
    if from == to {
        return 1;
    }

    let count = graph
        .get(from)
        .unwrap_or(&Vec::new())
        .iter()
        .map(|succ| {
            memo.get(succ)
                .copied()
                .unwrap_or_else(|| num_paths(succ, to, graph, memo))
        })
        .sum();

    memo.insert(from, count);
    count
}

pub fn part_1(input: &str) -> String {
    let graph = parse_input(input);
    num_paths("you", "out", &graph, &mut HashMap::new()).to_string()
}

pub fn part_2(input: &str) -> String {
    let graph = parse_input(input);
    let nodes_to_visit = match num_paths("fft", "dac", &graph, &mut HashMap::new()) {
        0 => ["svr", "dac", "fft", "out"],
        1.. => ["svr", "fft", "dac", "out"],
    };
    nodes_to_visit
        .iter()
        .tuple_windows()
        .map(|(from, to)| num_paths(from, to, &graph, &mut HashMap::new()))
        .product::<usize>()
        .to_string()
}
