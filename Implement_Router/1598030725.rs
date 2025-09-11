#[allow(dead_code)]
//struct Solution;

use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Eq, PartialEq, Hash, Clone)]
struct Packet {
    src: i32,
    dst: i32,
    time: i32,
}

struct DestinationStats {
    timeline: Vec<i32>,
    head: usize,
    tail: usize,
}

pub struct Router {
    capacity: usize,
    queue: VecDeque<Packet>,
    present: HashSet<Packet>,
    per_dest: HashMap<i32, DestinationStats>,
}

impl Router {
    pub fn new(limit: i32) -> Self {
        Router {
            capacity: limit as usize,
            queue: VecDeque::new(),
            present: HashSet::new(),
            per_dest: HashMap::new(),
        }
    }

    pub fn add_packet(&mut self, source: i32, destination: i32, timestamp: i32) -> bool {
        let pkt = Packet {
            src: source,
            dst: destination,
            time: timestamp,
        };

        if self.present.contains(&pkt) {
            return false;
        }

        if self.queue.len() >= self.capacity {
            if let Some(old) = self.queue.pop_front() {
                self.present.remove(&old);
                if let Some(entry) = self.per_dest.get_mut(&old.dst) {
                    entry.head += 1;
                }
            }
        }

        self.queue.push_back(pkt.clone());
        self.present.insert(pkt.clone());

        let entry = self.per_dest.entry(destination).or_insert_with(|| DestinationStats {
            timeline: Vec::new(),
            head: 0,
            tail: usize::MAX,
        });

        entry.timeline.push(timestamp);
        if entry.tail == usize::MAX {
            entry.tail = 0;
        } else {
            entry.tail += 1;
        }

        true
    }

    pub fn forward_packet(&mut self) -> Vec<i32> {
        if let Some(pkt) = self.queue.pop_front() {
            self.present.remove(&pkt);

            if let Some(entry) = self.per_dest.get_mut(&pkt.dst) {
                entry.head += 1;
            }

            vec![pkt.src, pkt.dst, pkt.time]
        } else {
            vec![]
        }
    }

    pub fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        let Some(stats) = self.per_dest.get(&destination) else {
            return 0;
        };

        if stats.head > stats.tail {
            return 0;
        }

        let slice = &stats.timeline[stats.head..=stats.tail];
        let lower = slice.partition_point(|&x| x < start_time);
        let upper = slice.partition_point(|&x| x <= end_time);

        (upper - lower) as i32
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