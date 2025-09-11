/*impl Solution {
    pub fn smallest_palindrome(s: String, k: i32) -> String {
        
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
impl Solution {
    pub fn smallest_palindrome(s: String, mut k: i32) -> String {
        const LIMIT: u64 = 1_000_000_000;
        let mut freq = [0; 26];
        s.chars().for_each(|c| freq[c as usize - 'a' as usize] += 1);

        let mut half = [0; 26];
        let mut total_half = 0;
        let mut center = None;

        for (i, &f) in freq.iter().enumerate() {
            half[i] = f / 2;
            total_half += half[i];
            if f % 2 == 1 {
                center.get_or_insert((b'a' + i as u8) as char);
            }
        }

        let mut state = half.to_vec();
        if Self::count_ways(&state, total_half, LIMIT) < k as u64 {
            return String::new();
        }

        let mut prefix = String::new();
        while total_half > 0 {
            for i in 0..26 {
                if state[i] == 0 {
                    continue;
                }

                state[i] -= 1;
                let options = Self::count_ways(&state, total_half - 1, LIMIT);

                if options >= k as u64 {
                    prefix.push((b'a' + i as u8) as char);
                    total_half -= 1;
                    break;
                } else {
                    k -= options as i32;
                    state[i] += 1;
                }
            }
        }

        let mut result = prefix.clone();
        if let Some(mid) = center {
            result.push(mid);
        }
        result.extend(prefix.chars().rev());
        result
    }

    fn count_ways(arr: &[i32], mut total: i32, limit: u64) -> u64 {
        let mut product = 1u64;
        for &val in arr.iter() {
            if val > 0 {
                product = product.saturating_mul(Self::comb(total, val, limit));
                if product > limit {
                    return limit;
                }
                total -= val;
            }
        }
        product
    }

    fn comb(n: i32, r: i32, limit: u64) -> u64 {
        if r > n || r < 0 {
            return 0;
        }

        let mut result = 1u64;
        let mut i = 1;
        let mut num = n - r + 1;

        while i <= r {
            result = result.saturating_mul(num as u64) / (i as u64);
            if result > limit {
                return limit;
            }
            i += 1;
            num += 1;
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