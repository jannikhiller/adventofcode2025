use itertools::*;

type I = u64;
type Point = [I; 3];

fn dist(a: &Point, b: &Point) -> I {
    izip!(a, b).map(|(&a, &b)| a.abs_diff(b).pow(2)).sum::<I>()
}

fn dfs(v: usize, graph: &[Vec<usize>], seen: &mut [bool]) -> usize {
    if seen[v] {
        return 0;
    }
    seen[v] = true;
    1 + graph[v].iter().map(|&v| dfs(v, graph, seen)).sum::<usize>()
}

fn parse_input(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|l| {
            l.split(",")
                .map(|c| c.parse().unwrap())
                .collect_array()
                .unwrap()
        })
        .collect()
}

pub fn part_1(input: &str) -> String {
    const NUM_CONNECTIONS: usize = 1000;
    let points = parse_input(input);

    let n = points.len();

    let closest_points = points
        .iter()
        .enumerate()
        .tuple_combinations()
        .k_smallest_relaxed_by_key(NUM_CONNECTIONS, |((_, p1), (_, p2))| dist(p1, p2))
        .map(|((i1, _), (i2, _))| (i1, i2));

    let mut graph = vec![vec![]; n];

    for (i1, i2) in closest_points {
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

pub fn part_2(input: &str) -> String {
    let points = parse_input(input);
    let n = points.len();

    let mut min_edge = vec![I::MAX; n];
    min_edge[0] = 0;

    let mut seen: Vec<bool> = vec![false; n];
    let mut parent: Vec<usize> = (0..n).collect();
    let mut largest_mst_edge = (0, 0, 0);

    loop {
        let candidates = min_edge.iter().enumerate().filter(|&(i, _)| !seen[i]);

        let Some((v, &d)) = candidates.min_by_key(|&(_, d)| d) else {
            break;
        };
        seen[v] = true;

        if d > largest_mst_edge.2 {
            largest_mst_edge = (parent[v], v, d)
        }

        for (w, p_w) in points.iter().enumerate() {
            let d_vw = dist(&points[v], p_w);
            if d_vw < min_edge[w] {
                min_edge[w] = d_vw;
                parent[w] = v;
            }
        }
    }

    let (v, w, _) = largest_mst_edge;
    (points[v][0] * points[w][0]).to_string()
}
