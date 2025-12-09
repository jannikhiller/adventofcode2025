use itertools::*;

type I = u64;
type Point = [I; 3];

fn dist(a: &Point, b: &Point) -> I {
    izip!(a.iter(), b.iter())
        .map(|(&a, &b)| a.abs_diff(b).pow(2))
        .sum::<I>()
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

fn find(mut v: usize, parent: &mut [usize]) -> usize {
    while parent[v] != v {
        parent[v] = parent[parent[v]];
        v = parent[v];
    }
    v
}

pub fn part_2(input: &str) -> String {
    let points = parse_input(input);
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
        .max_by_key(|t| t.2)
        .map(|(v, w, _)| points[v][0] * points[w][0])
        .unwrap()
        .to_string()
}
