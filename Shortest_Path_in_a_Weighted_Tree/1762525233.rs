use std::collections::VecDeque;

struct FenwickTree {
    size: usize,
    data: Vec<i64>,
}

impl FenwickTree {
    fn new(n: usize) -> Self {
        Self {
            size: n,
            data: vec![0; n + 2],
        }
    }

    fn add(&mut self, mut i: usize, delta: i64) {
        while i <= self.size {
            self.data[i] += delta;
            i += i & i.wrapping_neg();
        }
    }

    fn prefix_sum(&self, mut i: usize) -> i64 {
        let mut acc = 0;
        while i > 0 {
            acc += self.data[i];
            i &= i - 1;
        }
        acc
    }
}

impl Solution {
    pub fn tree_queries(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut graph = vec![vec![]; n + 1];
        for edge in &edges {
            let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2] as i64);
            graph[u].push((v, w));
            graph[v].push((u, w));
        }

        let mut time_in = vec![0; n + 1];
        let mut time_out = vec![0; n + 1];
        let mut parent = vec![0; n + 1];
        let mut distance = vec![0i64; n + 1];
        let mut edge_weight = vec![0i64; n + 1];

        let mut timestamp = 0;
        fn dfs(
            node: usize,
            prev: usize,
            dist: i64,
            graph: &Vec<Vec<(usize, i64)>>,
            parent: &mut Vec<usize>,
            distance: &mut Vec<i64>,
            edge_weight: &mut Vec<i64>,
            time_in: &mut Vec<usize>,
            time_out: &mut Vec<usize>,
            timer: &mut usize,
        ) {
            *timer += 1;
            time_in[node] = *timer;
            parent[node] = prev;
            distance[node] = dist;

            for &(next, weight) in &graph[node] {
                if next != prev {
                    edge_weight[next] = weight;
                    dfs(next, node, dist + weight, graph, parent, distance, edge_weight, time_in, time_out, timer);
                }
            }

            time_out[node] = *timer;
        }

        dfs(1, 0, 0, &graph, &mut parent, &mut distance, &mut edge_weight, &mut time_in, &mut time_out, &mut timestamp);

        let mut fenwick = FenwickTree::new(n);
        let mut result = vec![];

        for q in queries {
            match q[0] {
                1 => {
                    let (u, v, new_weight) = (q[1] as usize, q[2] as usize, q[3] as i64);
                    let child = if parent[u] == v { u } else { v };
                    let delta = new_weight - edge_weight[child];
                    edge_weight[child] = new_weight;

                    fenwick.add(time_in[child], delta);
                    fenwick.add(time_out[child] + 1, -delta);
                }
                2 => {
                    let x = q[1] as usize;
                    let total = distance[x] + fenwick.prefix_sum(time_in[x]);
                    result.push(total as i32);
                }
                _ => {}
            }
        }

        result
    }
}


