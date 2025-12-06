use std::cmp::max;
use std::ops::Range;

fn parse_input(input: &str) -> (impl Iterator<Item = Range<u64>>, impl Iterator<Item = u64>) {
    let (ranges, ids) = input.split_once("\n\n").unwrap();
    (
        ranges.lines().map(|pair| {
            let (a, b) = pair.split_once("-").unwrap();
            a.parse().unwrap()..b.parse::<u64>().unwrap() + 1
        }),
        ids.lines().map(|id| id.parse().unwrap()),
    )
}

fn combine_ranges(ranges: impl Iterator<Item = Range<u64>>) -> Vec<Range<u64>> {
    let mut sorted_ranges = ranges.collect::<Vec<_>>();
    sorted_ranges.sort_by_key(|&Range { start, .. }| start);

    let mut combined_ranges: Vec<Range<u64>> = Vec::new();
    let Some(&Range {
        start: mut curr_start,
        end: mut curr_end,
    }) = sorted_ranges.first()
    else {
        return Vec::new();
    };

    for Range { start, end } in sorted_ranges.into_iter().skip(1) {
        if start > curr_end {
            combined_ranges.push(curr_start..curr_end);
            curr_start = start;
            curr_end = end;
        } else {
            curr_end = max(curr_end, end);
        }
    }
    combined_ranges.push(curr_start..curr_end);
    combined_ranges
}

pub fn part_1(input: &str) -> String {
    let (ranges, ids) = parse_input(input);
    let ranges_vec = combine_ranges(ranges);
    ids.filter(|id| {
        let i = ranges_vec.partition_point(|Range { end, .. }| id >= end);
        ranges_vec.get(i).is_some_and(|r| r.contains(id))
    })
    .count()
    .to_string()
}

pub fn part_2(input: &str) -> String {
    let (ranges, _) = parse_input(input);
    combine_ranges(ranges)
        .into_iter()
        .map(|Range { start, end }| end - start)
        .sum::<u64>()
        .to_string()
}
