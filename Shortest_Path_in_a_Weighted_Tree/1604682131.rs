/*impl Solution {
    pub fn tree_queries(n: i32, edges: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        
    }
}*/

#[allow(dead_code)]

/*struct Solution;

impl Solution {

    pub fn your_function_name(input: Vec<i32>) -> Vec<i32> {

        #[cfg(debug_assertions)]

        logger::log_input(&input);

        let mut result = vec![];

        let mut tracker = helper::PrefixSum::from(&input);

        for i in 0..input.len() {

            let val = tracker.range_sum(0, i);

            result.push(val as i32);

        }

        #[cfg(debug_assertions)]

        logger::log_output(&result);

        result

    }

}*/
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


// === Logging utilities (disabled during submission) ===

#[cfg(debug_assertions)]

mod logger {

    pub fn log_input<T: std::fmt::Debug>(input: &T) {

        eprintln!("[Input]: {:?}", input);

    }

    pub fn log_output<T: std::fmt::Debug>(output: &T) {

        eprintln!("[Output]: {:?}", output);

    }

}

// === Helper Utilities ===

mod helper {

    pub fn gcd(mut a: i64, mut b: i64) -> i64 {

        while b != 0 {

            let t = b;

            b = a % b;

            a = t;

        }

        a

    }

    pub fn mod_pow(mut base: i64, mut exp: i64, modu: i64) -> i64 {

        let mut res = 1;

        base %= modu;

        while exp > 0 {

            if exp % 2 == 1 {

                res = res * base % modu;

            }

            base = base * base % modu;

            exp >>= 1;

        }

        res

    }

    pub fn mod_inv(a: i64, modu: i64) -> i64 {

        mod_pow(a, modu - 2, modu)

    }

    pub fn is_prime(n: i64) -> bool {

        if n < 2 {

            return false;

        }

        for i in 2..=((n as f64).sqrt() as i64) {

            if n % i == 0 {

                return false;

            }

        }

        true

    }

    pub struct PrefixSum {

        pub sum: Vec<i64>,

    }

    impl PrefixSum {

        pub fn from(data: &Vec<i32>) -> Self {

            let mut sum = vec![0; data.len() + 1];

            for (i, &val) in data.iter().enumerate() {

                sum[i + 1] = sum[i] + val as i64;

            }

            PrefixSum { sum }

        }

        pub fn range_sum(&self, l: usize, r: usize) -> i64 {

            self.sum[r + 1] - self.sum[l]

        }

    }

    pub struct UnionFind {

        parent: Vec<usize>,

        size: Vec<usize>,

    }

    impl UnionFind {

        pub fn new(n: usize) -> Self {

            UnionFind {

                parent: (0..n).collect(),

                size: vec![1; n],

            }

        }

        pub fn find(&mut self, x: usize) -> usize {

            if self.parent[x] != x {

                self.parent[x] = self.find(self.parent[x]);

            }

            self.parent[x]

        }

        pub fn union(&mut self, a: usize, b: usize) -> bool {

            let mut root_a = self.find(a);

            let mut root_b = self.find(b);

            if root_a == root_b {

                return false;

            }

            if self.size[root_a] < self.size[root_b] {

                std::mem::swap(&mut root_a, &mut root_b);

            }

            self.parent[root_b] = root_a;

            self.size[root_a] += self.size[root_b];

            true

        }

        pub fn same(&mut self, a: usize, b: usize) -> bool {

            self.find(a) == self.find(b)

        }

    }

    pub fn upper_bound(arr: &[i32], target: i32) -> usize {

        let mut low = 0;

        let mut high = arr.len();

        while low < high {

            let mid = (low + high) / 2;

            if arr[mid] <= target {

                low = mid + 1;

            } else {

                high = mid;

            }

        }

        low

    }

    pub fn lower_bound(arr: &[i32], target: i32) -> usize {

        let mut low = 0;

        let mut high = arr.len();

        while low < high {

            let mid = (low + high) / 2;

            if arr[mid] < target {

                low = mid + 1;

            } else {

                high = mid;

            }

        }

        low

    }

    pub fn reverse<T>(arr: &mut [T]) {

        let n = arr.len();

        for i in 0..n / 2 {

            arr.swap(i, n - 1 - i);

        }

    }

}

// === Graph Utilities ===

mod graph {

    use std::collections::VecDeque;

    pub fn bfs(start: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {

        let mut queue = VecDeque::new();

        queue.push_back(start);

        visited[start] = true;

        while let Some(u) = queue.pop_front() {

            for &v in &adj[u] {

                if !visited[v] {

                    visited[v] = true;

                    queue.push_back(v);

                }

            }

        }

    }

    pub fn dfs(u: usize, adj: &Vec<Vec<usize>>, visited: &mut Vec<bool>) {

        visited[u] = true;

        for &v in &adj[u] {

            if !visited[v] {

                dfs(v, adj, visited);

            }

        }

    }

    pub fn topological_sort(adj: &Vec<Vec<usize>>, n: usize) -> Vec<usize> {

        let mut indegree = vec![0; n];

        for u in 0..n {

            for &v in &adj[u] {

                indegree[v] += 1;

            }

        }

        let mut queue = VecDeque::new();

        for i in 0..n {

            if indegree[i] == 0 {

                queue.push_back(i);

            }

        }

        let mut order = vec![];

        while let Some(u) = queue.pop_front() {

            order.push(u);

            for &v in &adj[u] {

                indegree[v] -= 1;

                if indegree[v] == 0 {

                    queue.push_back(v);

                }

            }

        }

        order

    }

}