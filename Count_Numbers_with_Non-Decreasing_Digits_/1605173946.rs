/*impl Solution {
    pub fn count_numbers(l: String, r: String, b: i32) -> i32 {
        
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
    pub fn count_numbers(low: String, high: String, base: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        fn string_to_digits(mut num: String, base: i32) -> Vec<i32> {
            let mut result = Vec::new();
            while num != "0" {
                let mut remainder = 0;
                let mut next = String::new();

                for ch in num.chars() {
                    let digit = ch as i32 - '0' as i32;
                    let current = remainder * 10 + digit;
                    let quotient = current / base;
                    remainder = current % base;
                    if !next.is_empty() || quotient != 0 {
                        next.push((quotient as u8 + b'0') as char);
                    }
                }

                result.push(remainder);
                num = if next.is_empty() { "0".to_string() } else { next };
            }

            if result.is_empty() {
                result.push(0);
            }

            result.reverse();
            result
        }

        fn decrement(num: String) -> String {
            let mut chars: Vec<char> = num.chars().collect();
            let mut idx = chars.len();
            while idx > 0 && chars[idx - 1] == '0' {
                chars[idx - 1] = '9';
                idx -= 1;
            }
            if idx > 0 {
                chars[idx - 1] = ((chars[idx - 1] as u8) - 1) as char;
            }

            while chars.len() > 1 && chars[0] == '0' {
                chars.remove(0);
            }

            chars.iter().collect()
        }

        fn solve(
            pos: usize,
            tight: bool,
            started: bool,
            prev_digit: usize,
            digits: &Vec<i32>,
            base: i32,
            memo: &mut Vec<Vec<Vec<Vec<i64>>>>,
        ) -> i64 {
            if pos == digits.len() {
                return 1;
            }

            let t = if tight { 1 } else { 0 };
            let s = if started { 1 } else { 0 };

            if memo[pos][t][s][prev_digit] != -1 {
                return memo[pos][t][s][prev_digit];
            }

            let limit = if tight { digits[pos] } else { base - 1 };
            let mut total = 0i64;

            for d in 0..=limit {
                if started && d < prev_digit as i32 {
                    continue;
                }

                let next_tight = tight && (d == limit);
                let next_started = started || d > 0;
                let next_prev = if next_started { d as usize } else { 0 };

                total = (total + solve(
                    pos + 1,
                    next_tight,
                    next_started,
                    next_prev,
                    digits,
                    base,
                    memo,
                )) % MOD;
            }

            memo[pos][t][s][prev_digit] = total;
            total
        }

        fn count_up_to(num: String, base: i32) -> i64 {
            let digits = string_to_digits(num, base);
            let len = digits.len();
            let mut memo = vec![vec![vec![vec![-1; base as usize]; 2]; 2]; len];
            solve(0, true, false, 0, &digits, base, &mut memo)
        }

        let total_high = count_up_to(high.clone(), base);
        let total_low = if low == "0" {
            0
        } else {
            let low_minus_one = decrement(low);
            count_up_to(low_minus_one, base)
        };

        ((total_high - total_low + MOD) % MOD) as i32
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