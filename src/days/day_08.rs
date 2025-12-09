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

#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, mut v: usize) -> usize {
        while self.parent[v] != v {
            self.parent[v] = self.parent[self.parent[v]];
            v = self.parent[v];
        }
        v
    }

    fn union(&mut self, mut v: usize, mut w: usize) -> bool {
        v = self.find(v);
        w = self.find(w);
        if v == w {
            return false;
        }
        if self.size[v] < self.size[w] {
            (v, w) = (w, v);
        }
        self.parent[w] = v;
        self.size[v] += self.size[w];
        true
    }

    fn num_subsets(&self) -> usize {
        self.parent
            .iter()
            .enumerate()
            .filter(|&(v, &p)| v == p)
            .count()
    }
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
    let dist = |v: usize, w: usize| dist(&points[v], &points[w]);

    let mut components = UnionFind::new(n);
    let mut mst_edges = vec![];

    // for my input only doing one iteration of this loop would be enough
    loop {
        let mut cheapest_edge = vec![None; n];
        for (v, w) in (0..n).tuple_combinations() {
            let v_root = components.find(v);
            let w_root = components.find(w);
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
                if components.union(v, w) {
                    mst_edges.push((v, w, dist_vw))
                }
            }
        }

        if components.num_subsets() == 1 {
            break;
        }
    }

    mst_edges
        .into_iter()
        .max_by_key(|t| t.2)
        .map(|(v, w, _)| points[v][0] * points[w][0])
        .unwrap()
        .to_string()
}
