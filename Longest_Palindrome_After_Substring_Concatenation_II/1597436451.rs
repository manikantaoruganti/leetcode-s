/*impl Solution {
    pub fn longest_palindrome(s: String, t: String) -> i32 {
        
    }
}*/
 #[allow(dead_code)]

//struct Solution;
impl Solution {
    pub fn longest_palindrome(s: String, t: String) -> i32 {
        #[cfg(debug_assertions)]
        logger::log_strs(&[&s, &t]);

        let n = s.len();
        let m = t.len();
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();

        let mut pal_s = vec![vec![false; n]; n];
        let mut pal_t = vec![vec![false; m]; m];
        
        for i in 0..n {
            pal_s[i][i] = true;
        }
        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                if s_chars[i] == s_chars[j] && (len == 2 || pal_s[i + 1][j - 1]) {
                    pal_s[i][j] = true;
                }
            }
        }

        for i in 0..m {
            pal_t[i][i] = true;
        }
        for len in 2..=m {
            for i in 0..=m - len {
                let j = i + len - 1;
                if t_chars[i] == t_chars[j] && (len == 2 || pal_t[i + 1][j - 1]) {
                    pal_t[i][j] = true;
                }
            }
        }
        
        let mut lps_start = vec![0; n];
        for i in 0..n {
            for j in i..n {
                if pal_s[i][j] {
                    lps_start[i] = lps_start[i].max(j - i + 1);
                }
            }
        }
        
        let mut lps_end = vec![0; m];
        for j in 0..m {
            for i in 0..=j {
                if pal_t[i][j] {
                    lps_end[j] = lps_end[j].max(j - i + 1);
                }
            }
        }
        
        let mut answer = *lps_start.iter().max().unwrap().max(lps_end.iter().max().unwrap());
        let mut rev_t = t_chars.clone();
        rev_t.reverse();
        
        let mut dp = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                if s_chars[i] == rev_t[j] {
                    dp[i][j] = if i == 0 || j == 0 { 1 } else { dp[i - 1][j - 1] + 1 };
                    let l = dp[i][j];
                    let start_s = i + 1 - l;
                    let t_start = m - 1 - j;
                    if t_start < 0 { continue; }
                    let cand1 = 2 * l + if start_s + l < n { lps_start[start_s + l] } else { 0 };
                    let cand2 = 2 * l + if t_start > 0 { lps_end[t_start - 1] } else { 0 };
                    answer = answer.max(cand1.max(cand2));
                }
            }
        }

        #[cfg(debug_assertions)]
        logger::log_value("answer", answer);

        answer as i32
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