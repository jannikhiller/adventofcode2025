use std::{
    cmp::{Ord, Ordering},
    collections::BinaryHeap,
};

use crate::util::*;
use itertools::*;

type I = u64;
type F = f32;

#[derive(PartialEq, PartialOrd, Clone)]
struct FWrapper(F);

impl Eq for FWrapper {}

impl Ord for FWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

fn dist(a: &[I], b: &[I]) -> F {
    izip!(a.iter(), b.iter())
        .map(|(&a, &b)| a.abs_diff(b).pow(2) as F)
        .sum::<F>()
        .sqrt()
}

fn dfs(v: usize, graph: &[Vec<usize>], seen: &mut [bool]) -> usize {
    if seen[v] {
        return 0;
    }
    seen[v] = true;
    1 + graph[v].iter().map(|&v| dfs(v, graph, seen)).sum::<usize>()
}

pub fn part_1(input: &str) -> String {
    const NUM_CONNECTIONS: usize = 1000;

    let points: Vec<Vec<I>> = input.parse_grid_rows_with_splitter(|l| l.split(","));
    let n = points.len();

    let mut closest_points = BinaryHeap::new();
    for ((i1, p1), (i2, p2)) in points.iter().enumerate().tuple_combinations() {
        closest_points.push((FWrapper(dist(p1, p2)), i1, i2));
        if closest_points.len() > NUM_CONNECTIONS {
            closest_points.pop();
        }
    }

    let mut graph = vec![vec![]; n];

    for (_, i1, i2) in closest_points {
        graph[i1].push(i2);
        graph[i2].push(i1);
    }

    let mut seen = vec![false; n];

    (0..n)
        .map(|v| dfs(v, &graph, &mut seen))
        .k_largest(3)
        .product::<usize>()
        .to_string()
}

fn find(mut v: usize, parent: &mut [usize]) -> usize {
    while parent[v] != v {
        parent[v] = parent[parent[v]];
        v = parent[v];
    }
    v
}

pub fn part_2(input: &str) -> String {
    let points: Vec<Vec<I>> = input.parse_grid_rows_with_splitter(|l| l.split(","));
    let n = points.len();
    let dist = |v: usize, w: usize| dist(&points[v], &points[w]);

    let mut parent: Vec<usize> = (0..n).collect();

    let mut changed = true;
    let mut mst_edges = vec![];

    while changed {
        changed = false;
        let mut cheapest_edge = vec![None; n];
        for (v, w) in (0..n).tuple_combinations() {
            let v_root = find(v, &mut parent);
            let w_root = find(w, &mut parent);
            if v_root == w_root {
                continue;
            }

            let dist_vw = dist(v, w);

            cheapest_edge[v_root] = match cheapest_edge[v_root] {
                Some((_, dist_vx)) if dist_vw < dist_vx => Some((w, dist_vw)),
                Some(t) => Some(t),
                None => Some((w, dist_vw)),
            };

            cheapest_edge[w_root] = match cheapest_edge[w_root] {
                Some((_, dist_wx)) if dist_vw < dist_wx => Some((v, dist_vw)),
                Some(t) => Some(t),
                None => Some((v, dist_vw)),
            }
        }

        for (v, opt) in cheapest_edge.into_iter().enumerate() {
            if let Some((w, dist_vw)) = opt {
                changed = true;
                let v_root = find(v, &mut parent);
                let w_root = find(w, &mut parent);
                if v_root == w_root {
                    continue;
                }
                parent[v_root] = w_root;
                mst_edges.push((v, w, dist_vw))
            }
        }
    }

    mst_edges
        .into_iter()
        .max_by_key(|t| FWrapper(t.2))
        .map(|(v, w, _)| points[v][0] * points[w][0])
        .unwrap()
        .to_string()
}
