use crate::util::*;
use itertools::*;

use std::cmp::Reverse;

type I = u64;
type Point = [I; 2];

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

fn in_order(a: I, b: I, c: I) -> bool {
    (a < b && b < c) || (a > b && b > c)
}

fn line_segments_intersect(line_a: (&Point, &Point), line_b: (&Point, &Point)) -> bool {
    let (mut p1_a, mut p2_a) = line_a;
    let (mut p1_b, mut p2_b) = line_b;

    if (p1_a[0] == p2_a[0]) == (p1_b[0] == p2_b[0]) {
        // lines are parallel
        return false;
    }
    // make first line be horizontal
    if p1_a[0] != p2_a[0] {
        ((p1_a, p2_a), (p1_b, p2_b)) = ((p1_b, p2_b), (p1_a, p2_a))
    }
    in_order(p1_a[1], p1_b[1], p2_a[1]) && in_order(p1_b[0], p1_a[0], p2_b[0])
}

fn line_midpoint_is_in_rect(rect: (&Point, &Point), line: (&Point, &Point)) -> bool {
    //scale everything by two, so there is a exact midpoint
    let (rect_p1, rect_p2) = (
        [rect.0[0] * 2, rect.0[1] * 2],
        [rect.1[0] * 2, rect.1[1] * 2],
    );
    let (line_p1, line_p2) = (
        [line.0[0] * 2, line.0[1] * 2],
        [line.1[0] * 2, line.1[1] * 2],
    );
    let midpoint: Point = [(line_p1[0] + line_p2[0]) / 2, (line_p1[1] + line_p2[1]) / 2];
    in_order(rect_p1[0], midpoint[0], rect_p2[0]) && in_order(rect_p1[1], midpoint[1], rect_p2[1])
}

fn rectangle_intersects_line_segment(
    (p_1, p_2): (&Point, &Point),
    line_segments: &[Point],
) -> bool {
    let corners = [(p_1, p_1), (p_1, p_2), (p_2, p_2), (p_2, p_1)]
        .into_iter()
        .map(|(p_a, p_b)| [p_a[0], p_b[1]])
        .collect_array::<4>()
        .unwrap();

    line_segments
        .iter()
        .circular_tuple_windows::<(_, _)>()
        .any(|line_segment| {
            corners
                .iter()
                .circular_tuple_windows::<(_, _)>()
                .any(|rect_line_segment| line_segments_intersect(rect_line_segment, line_segment))
                || line_midpoint_is_in_rect((p_1, p_2), line_segment)
        })
}

pub fn part_2(input: &str) -> String {
    let points = input.parse_points::<I, 2>();
    let rects = points
        .iter()
        .tuple_combinations()
        .map(|(p1, p2)| {
            let size = izip!(p1, p2)
                .map(|(x1, x2)| x1.abs_diff(*x2) + 1)
                .product::<I>();
            (p1, p2, size)
        })
        .sorted_unstable_by_key(|(_, _, size)| Reverse(*size));

    rects
        .filter_map(|(p1, p2, size)| {
            (!rectangle_intersects_line_segment((p1, p2), &points)).then_some(size)
        })
        .next()
        .unwrap()
        .to_string()
}
