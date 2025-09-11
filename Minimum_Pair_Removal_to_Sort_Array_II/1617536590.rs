/*impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        
    }
}*/

/*use std::collections::*;

use std::cmp::*;

use std::io::*;
use std::f64;
use std::fmt::*;
use std::hash::*;
use std::str::*;
use std::time::*;
use std::ops::*;
use std::mem::*;*/

//use std::io::{self, BufRead, Write};
use std::collections::{VecDeque, HashMap, /*HashSet*/}; // whatever collections you use
/*use std::collections::{ BTreeSet};*/

use std::str::FromStr;
use std::fmt::Debug;

// ---------- Macros ----------
/*macro_rules! debug {
    ($($x:expr),*) => {
        eprint!("[DEBUG] {}:{} - ", file!(), line!());
        $(eprint!("{:?} ", $x);)*
        eprintln!();
    }
}

macro_rules! FOR {
    ($i:ident, $a:expr, $b:expr) => {
        for $i in $a..$b
    };
}

macro_rules! FORR {
    ($i:ident, $a:expr, $b:expr) => {
        for $i in ($b..=$a).rev()
    };
}

macro_rules! REP {
    ($i:ident, $n:expr) => {
        for $i in 0..$n
    };
}

macro_rules! REPR {
    ($i:ident, $n:expr) => {
        for $i in (0..$n).rev()
    };
}
*/
// ---------- Constants ----------
const MOD: i64 = 1_000_000_007;
const INF: i64 = 1_000_000_000;
const LINF: i64 = 1_000_000_000_000_000_000;
const PI: f64 = std::f64::consts::PI;
const MAXN: usize = 200_005;
// ---------- Type Definitions ----------
type I = i32;
type II = i64;
type U = u32;
type UU = u64;
type F = f32;
type FF = f64;

type VI = Vec<I>;
type VII = Vec<II>;
type VU = Vec<U>;
type VUU = Vec<UU>;
type VF = Vec<F>;
type VFF = Vec<FF>;
type VB = Vec<bool>;
type VS = Vec<String>;

type PII = (I, I);
type PIIII = (II, II);

type VPI = Vec<PII>;
type VPIIII = Vec<PIIII>;

type VVI = Vec<VI>;
type VVII = Vec<VII>;
type VVU = Vec<VU>;
type VVUU = Vec<VUU>;
type VVF = Vec<VF>;
type VVFF = Vec<VFF>;
type VVB = Vec<VB>;
type VVS = Vec<VS>;
// ---------- Math Functions ----------
fn gcd( a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

fn modpow(mut a: i64, mut b: i64, m: i64) -> i64 {
    let mut res = 1;
    a %= m;
    while b > 0 {
        if b & 1 == 1 {
            res = res * a % m;
        }
        a = a * a % m;
        b >>= 1;
    }
    res
}

fn modinv(a: i64, m: i64) -> i64 {
    modpow(a, m - 2, m)
}

fn ncr(n: usize, r: usize, fact: &Vec<i64>, inv_fact: &Vec<i64>) -> i64 {
    if r > n || r < 0 {
        return 0;
    }
    fact[n] * inv_fact[r] % MOD * inv_fact[n - r] % MOD
}

// ---------- Prime Number Utilities ----------
fn sieve(n: usize) -> Vec<i64> {
    let mut primes = vec![];
    let mut is_prime = vec![true; n + 1];
    if n >= 1 {
        is_prime[0] = false;
        is_prime[1] = false;
    }
    for i in 2..=n {
        if is_prime[i] {
            primes.push(i as i64);
            let mut j = i * 2;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    primes
}

fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}
// ---------- Combinatorics ----------
fn factorial(n: usize, m: i64) -> Vec<i64> {
    let mut fact = vec![1; n + 1];
    for i in 1..=n {
        fact[i] = fact[i - 1] * i as i64 % m;
    }
    fact
}

fn inverse_factorial(n: usize, m: i64) -> Vec<i64> {
    let mut inv_fact = vec![0; n + 1];
    let fact = factorial(n, m);
    inv_fact[n] = modinv(fact[n], m);
    for i in (0..n).rev() {
        inv_fact[i] = inv_fact[i + 1] * (i as i64 + 1) % m;
    }
    inv_fact
}

// ---------- Disjoint Set Union (DSU) ----------
struct DSU {
    parent: Vec<i32>,
    rank: Vec<i32>,
}

impl DSU {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for i in 0..n {
            parent[i] = i as i32;
        }
        let rank = vec![1; n];
        Self { parent, rank }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] as usize != x {
            self.parent[x] = self.find(self.parent[x] as usize) as i32;
        }
        self.parent[x] as usize
    }

    fn unite(&mut self, x: usize, y: usize) {
        let  x_root = self.find(x);
        let  y_root = self.find(y);
        if x_root != y_root {
            if self.rank[x_root] < self.rank[y_root] {
                self.parent[x_root] = y_root as i32;
            } else {
                self.parent[y_root] = x_root as i32;
                if self.rank[x_root] == self.rank[y_root] {
                    self.rank[x_root] += 1;
                }
            }
        }
    }
}

// ---------- Fenwick Tree (BIT) ----------
struct FenwickTree {
    bit: Vec<i64>,
    n: usize,
}

impl FenwickTree {
    fn new(size: usize) -> Self {
        Self {
            bit: vec![0; size + 1],
            n: size,
        }
    }

    fn update(&mut self, mut idx: usize, delta: i64) {
        while idx <= self.n {
            self.bit[idx] += delta;
            idx += idx & (!idx + 1);
        }
    }

    fn query(&self, mut idx: usize) -> i64 {
        let mut res = 0;
        while idx > 0 {
            res += self.bit[idx];
            idx -= idx & (!idx + 1);
        }
        res
    }

    fn range_query(&self, l: usize, r: usize) -> i64 {
        self.query(r) - self.query(l - 1)
    }
}

// ---------- Segment Tree ----------
struct SegmentTree {
    tree: Vec<i64>,
    size: usize,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let mut size = 1;
        while size < n {
            size <<= 1;
        }
        Self {
            tree: vec![0; 2 * size],
            size,
        }
    }

    fn build(&mut self, a: &Vec<i64>, x: usize, lx: usize, rx: usize) {
        if rx - lx == 1 {
            if lx < a.len() {
                self.tree[x] = a[lx];
            }
            return;
        }
        let m = (lx + rx) / 2;
        self.build(a, 2 * x + 1, lx, m);
        self.build(a, 2 * x + 2, m, rx);
        self.tree[x] = self.tree[2 * x + 1] + self.tree[2 * x + 2];
    }

    fn build_from(&mut self, a: &Vec<i64>) {
        self.build(a, 0, 0, self.size);
    }

    fn update(&mut self, idx: usize, val: i64, x: usize, lx: usize, rx: usize) {
        if rx - lx == 1 {
            self.tree[x] = val;
            return;
        }
        let m = (lx + rx) / 2;
        if idx < m {
            self.update(idx, val, 2 * x + 1, lx, m);
        } else {
            self.update(idx, val, 2 * x + 2, m, rx);
        }
        self.tree[x] = self.tree[2 * x + 1] + self.tree[2 * x + 2];
    }

    fn update_at(&mut self, idx: usize, val: i64) {
        self.update(idx, val, 0, 0, self.size);
    }

    fn query(&self, l: usize, r: usize, x: usize, lx: usize, rx: usize) -> i64 {
        if lx >= r || rx <= l {
            return 0;
        }
        if lx >= l && rx <= r {
            return self.tree[x];
        }
        let m = (lx + rx) / 2;
        let left = self.query(l, r, 2 * x + 1, lx, m);
        let right = self.query(l, r, 2 * x + 2, m, rx);
        left + right
    }

    fn range_query(&self, l: usize, r: usize) -> i64 {
        self.query(l, r, 0, 0, self.size)
    }
}
// ---------- Heavy-Light Decomposition ----------


// ---------- String Algorithms ----------
fn prefix_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let s = s.as_bytes();
    let mut pi = vec![0; n];
    for i in 1..n {
        let mut j = pi[i - 1];
        while j > 0 && s[i] != s[j] {
            j = pi[j - 1];
        }
        if s[i] == s[j] {
            j += 1;
        }
        pi[i] = j;
    }
    pi
}

fn z_function(s: &str) -> Vec<usize> {
    let n = s.len();
    let s = s.as_bytes();
    let mut z = vec![0; n];
    let (mut l, mut r) = (0, 0);
    for i in 1..n {
        if i <= r {
            z[i] = (r - i + 1).min(z[i - l]);
        }
        while i + z[i] < n && s[z[i]] == s[i + z[i]] {
            z[i] += 1;
        }
        if i + z[i] - 1 > r {
            l = i;
            r = i + z[i] - 1;
        }
    }
    z
}
struct HLD {
    n: usize,
    timer: usize,
    parent: Vec<isize>,
    depth: Vec<usize>,
    heavy: Vec<isize>,
    head: Vec<usize>,
    pos: Vec<usize>,
    adj: Vec<Vec<usize>>,
}

impl HLD {
    fn new(size: usize) -> Self {
        HLD {
            n: size,
            timer: 0,
            parent: vec![-1; size],
            depth: vec![0; size],
            heavy: vec![-1; size],
            head: vec![0; size],
            pos: vec![0; size],
            adj: vec![vec![]; size],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
        self.adj[v].push(u);
    }

    fn dfs(&mut self, v: usize) -> usize {
        let mut size = 1;
        let mut max_c_size = 0;

        let children: Vec<usize> = self.adj[v]
            .iter()
            .cloned()
            .filter(|&c| c as isize != self.parent[v])
            .collect();

        for c in children {
            self.parent[c] = v as isize;
            self.depth[c] = self.depth[v] + 1;
            let c_size = self.dfs(c);
            size += c_size;
            if c_size > max_c_size {
                max_c_size = c_size;
                self.heavy[v] = c as isize;
            }
        }

        size
    }

 fn decompose(&mut self, v: usize, h: usize) {
    self.head[v] = h;
    self.pos[v] = self.timer;
    self.timer += 1;

    if self.heavy[v] != -1 {
        self.decompose(self.heavy[v] as usize, h);
    }

    // Create a temporary copy of the adjacent nodes to avoid borrowing issues
    let adj_v = self.adj[v].clone();
    for &c in &adj_v {
        if c as isize != self.parent[v] && Some(c) != self.heavy[v].try_into().ok() {
            self.decompose(c, c);
        }
    }
}   
fn build(&mut self, root: usize) {
        self.dfs(root);
        self.decompose(root, root);
    }
}
// ---------- Graph Algorithms ----------
fn floyd_warshall(mut dist: Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let n = dist.len();
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
            }
        }
    }
    dist
}

fn topological_sort(adj: &[Vec<usize>]) -> Option<Vec<usize>> {
    let n = adj.len();
    let mut in_deg = vec![0; n];
    for u in 0..n {
        for &v in &adj[u] {
            in_deg[v] += 1;
        }
    }
    let mut q = VecDeque::new();
    for i in 0..n {
        if in_deg[i] == 0 {
            q.push_back(i);
        }
    }
    let mut order = vec![];
    while let Some(u) = q.pop_front() {
        order.push(u);
        for &v in &adj[u] {
            in_deg[v] -= 1;
            if in_deg[v] == 0 {
                q.push_back(v);
            }
        }
    }
    if order.len() == n {
        Some(order)
    } else {
        None
    }
}

// ---------- Binary Search Utilities ----------
fn lower_bound<T: Ord>(v: &[T], x: &T) -> usize {
    v.binary_search(x).unwrap_or_else(|i| i)
}

fn upper_bound<T: Ord>(v: &[T], x: &T) -> usize {
    match v.binary_search(x) {
        Ok(mut i) => {
            while i < v.len() && &v[i] == x {
                i += 1;
            }
            i
        }
        Err(i) => i,
    }
}

fn is_square(x: i64) -> bool {
    let s = (x as f64).sqrt() as i64;
    s * s == x || (s + 1) * (s + 1) == x
}

// ---------- I/O Utilities ----------


fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line.trim().to_string()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
    read_line()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn print_vector<T: std::fmt::Display>(v: &[T], newline: bool) {
    for x in v {
        print!("{} ", x);
    }
    if newline {
        println!();
    }
}
//your function for submitting 
// ---------- Main Function ----------
const MODULO: i64 = 998_244_353;

fn power_mod(mut base: i64, mut exponent: i64, modulus: i64) -> i64 {
    let mut result = 1;
    base %= modulus;
    while exponent > 0 {
        if exponent & 1 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exponent >>= 1;
    }
    result
}

// ---------- Bit Manipulation Utilities ----------
fn count_bits(x: i64) -> i64 {
    let mut count = 0;
    let mut num = x;
    while num > 0 {
        count += num & 1;
        num >>= 1;
    }
    count
}

fn bitwise_or(a: i64, b: i64) -> i64 {
    a | b
}

fn bitwise_and(a: i64, b: i64) -> i64 {
    a & b
}

fn bitwise_xor(a: i64, b: i64) -> i64 {
    a ^ b
}

fn bitwise_not(a: i64) -> i64 {
    !a
}

fn is_bit_set(x: i64, pos: usize) -> bool {
    (x & (1 << pos)) != 0
}

fn set_bit(x: i64, pos: usize) -> i64 {
    x | (1 << pos)
}

fn clear_bit(x: i64, pos: usize) -> i64 {
    x & !(1 << pos)
}

fn toggle_bit(x: i64, pos: usize) -> i64 {
    x ^ (1 << pos)
}

// ---------- Matrix Operations ----------
fn mat_mult(a: &Vec<Vec<i64>>, b: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let n = a.len();
    let m = b[0].len();
    let mut result = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            for k in 0..a[0].len() {
                result[i][j] = (result[i][j] + a[i][k] * b[k][j]) % MOD;
            }
        }
    }
    result
}

fn mat_pow(a: &Vec<Vec<i64>>, p: usize) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut result = vec![vec![0; n]; n];
    for i in 0..n {
        result[i][i] = 1;
    }
    let mut base = a.clone();
    let mut exp = p;
    while exp > 0 {
        if exp % 2 == 1 {
            result = mat_mult(&result, &base);
        }
        base = mat_mult(&base, &base);
        exp /= 2;
    }
    result
}

// ---------- Trie Data Structure ----------
struct Trie {
    children: [Option<Box<Trie>>; 26],
    is_end_of_word: bool,
}

impl Trie {
    fn new() -> Self {
        Trie {
            children: Default::default(),
            is_end_of_word: false,
        }
    }

    fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            let idx = (c as u8 - b'a') as usize;
            if node.children[idx].is_none() {
                node.children[idx] = Some(Box::new(Trie::new()));
            }
            node = node.children[idx].as_mut().unwrap();
        }
        node.is_end_of_word = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut node = self;
        for c in word.chars() {
            let idx = (c as u8 - b'a') as usize;
            if node.children[idx].is_none() {
                return false;
            }
            node = node.children[idx].as_ref().unwrap();
        }
        node.is_end_of_word
    }

    fn starts_with(&self, prefix: &str) -> bool {
        let mut node = self;
        for c in prefix.chars() {
            let idx = (c as u8 - b'a') as usize;
            if node.children[idx].is_none() {
                return false;
            }
            node = node.children[idx].as_ref().unwrap();
        }
        true
    }
}

// ---------- Hashing Utilities ----------
fn string_hash(s: &str) -> i64 {
    let mut hash = 0;
    let p: i64 = 31;
    let mod_val: i64 = 1_000_000_007;
    let mut p_pow = 1;
    for c in s.chars() {
        hash = (hash + (c as i64 - 'a' as i64 + 1) * p_pow) % mod_val;
        p_pow = (p_pow * p) % mod_val;
    }
    hash
}

fn array_hash(arr: &[i64]) -> i64 {
    let mut hash = 0;
    let p: i64 = 31;
    let mod_val: i64 = 1_000_000_007;
    let mut p_pow = 1;
    for &val in arr.iter() {
        hash = (hash + (val * p_pow)) % mod_val;
        p_pow = (p_pow * p) % mod_val;
    }
    hash
}

// ---------- Queue Utilities ----------
fn bfs(start: usize, adj: &Vec<Vec<usize>>) -> Vec<usize> {
    let n = adj.len();
    let mut dist = vec![-1; n];
    let mut queue = VecDeque::new();
    queue.push_back(start);
    dist[start] = 0;

    while let Some(u) = queue.pop_front() {
        for &v in &adj[u] {
            if dist[v] == -1 {
                dist[v] = dist[u] + 1;
                queue.push_back(v);
            }
        }
    }

    dist.iter().map(|&d| d as usize).collect()
}

// ---------- Miscellaneous ----------
fn is_palindrome(s: &str) -> bool {
    let s: Vec<char> = s.chars().collect();
    let n = s.len();
    for i in 0..n / 2 {
        if s[i] != s[n - i - 1] {
            return false;
        }
    }
    true
}

fn generate_combinations(n: usize, r: usize) -> Vec<Vec<usize>> {
    let mut res = Vec::new();
    let mut comb = vec![0; r];
    let mut idx = 0;
    while idx < r {
        comb[idx] += 1;
        if comb[idx] == n - r + idx + 1 {
            idx -= 1;
        } else {
            idx += 1;
            if idx == r {
                res.push(comb.clone());
                idx -= 1;
            }
        }
    }
    res
}   




// ========== ADVANCED GRAPH ALGORITHMS ==========
fn topo_sort(n: usize, adj: &[Vec<usize>]) -> Vec<usize> {
    let mut indeg = vec![0; n];
    for v in adj.iter().flatten() {
        indeg[*v] += 1;
    }
    let mut q = std::collections::VecDeque::new();
    for i in 0..n {
        if indeg[i] == 0 {
            q.push_back(i);
        }
    }
    let mut res = Vec::new();
    while let Some(u) = q.pop_front() {
        res.push(u);
        for &v in &adj[u] {
            indeg[v] -= 1;
            if indeg[v] == 0 {
                q.push_back(v);
            }
        }
    }
    res
}

fn kosaraju_scc(n: usize, adj: &[Vec<usize>]) -> Vec<usize> {
    let mut order = Vec::new();
    let mut used = vec![false; n];
    fn dfs(v: usize, adj: &[Vec<usize>], used: &mut [bool], order: &mut Vec<usize>) {
        used[v] = true;
        for &u in &adj[v] {
            if !used[u] {
                dfs(u, adj, used, order);
            }
        }
        order.push(v);
    }

    for i in 0..n {
        if !used[i] {
            dfs(i, adj, &mut used, &mut order);
        }
    }

    let mut radj = vec![vec![]; n];
    for v in 0..n {
        for &u in &adj[v] {
            radj[u].push(v);
        }
    }

    let mut comp = vec![0; n];
    let mut cur = 0;
    used.fill(false);
    fn dfs_rev(v: usize, g: &[Vec<usize>], used: &mut [bool], comp: &mut [usize], label: usize) {
        used[v] = true;
        comp[v] = label;
        for &u in &g[v] {
            if !used[u] {
                dfs_rev(u, g, used, comp, label);
            }
        }
    }

    for &v in order.iter().rev() {
        if !used[v] {
            dfs_rev(v, &radj, &mut used, &mut comp, cur);
            cur += 1;
        }
    }
    comp
}

// ========== MATH UTILS AND COMBINATORICS ==========
fn modin(mut a: i64, m: i64) -> i64 {
    let (mut m0, mut x0, mut x1) = (m, 0, 1);
    if m == 1 { return 0; }
    while a > 1 {
        let q = a / m0;
        let t = m0;
        m0 = a % m0;
        a = t;
        let t = x0;
        x0 = x1 - q * x0;
        x1 = t;
    }
    if x1 < 0 { x1 += m; }
    x1
}

fn fast_pow(mut base: i64, mut exp: i64, m: i64) -> i64 {
    let mut res = 1;
    base %= m;
    while exp > 0 {
        if exp & 1 == 1 {
            res = res * base % m;
        }
        base = base * base % m;
        exp >>= 1;
    }
    res
}

fn precompute_combinations(n: usize, m: i64) -> (Vec<i64>, Vec<i64>) {
    let mut fact = vec![1; n + 1];
    for i in 1..=n {
        fact[i] = fact[i - 1] * (i as i64) % m;
    }
    let mut inv = vec![1; n + 1];
    inv[n] = modinv(fact[n], m);
    for i in (1..n).rev() {
        inv[i] = inv[i + 1] * ((i + 1) as i64) % m;
    }
    (fact, inv)
}

fn comb(n: usize, k: usize, fact: &[i64], inv: &[i64], m: i64) -> i64 {
    if k > n { return 0; }
    fact[n] * inv[k] % m * inv[n - k] % m
}


// ========== BINARY LIFTING FOR LCA ==========
struct BinaryLifting {
    up: Vec<Vec<usize>>,
    depth: Vec<usize>,
    log: usize,
}

impl BinaryLifting {
    fn new(n: usize, root: usize, tree: &[Vec<usize>]) -> Self {
        let log = 64 - n.leading_zeros() as usize;
        let mut up = vec![vec![0; n]; log];
        let mut depth = vec![0; n];
        fn dfs(u: usize, p: usize, d: usize, tree: &[Vec<usize>], up: &mut Vec<Vec<usize>>, depth: &mut Vec<usize>) {
            up[0][u] = p;
            depth[u] = d;
            for &v in &tree[u] {
                if v != p {
                    dfs(v, u, d + 1, tree, up, depth);
                }
            }
        }
        dfs(root, root, 0, tree, &mut up, &mut depth);
        for k in 1..log {
            for i in 0..n {
                up[k][i] = up[k - 1][up[k - 1][i]];
            }
        }
        Self { up, depth, log }
    }

    fn lca(&self, mut u: usize, mut v: usize) -> usize {
        if self.depth[u] < self.depth[v] {
            std::mem::swap(&mut u, &mut v);
        }
        for k in (0..self.log).rev() {
            if self.depth[u] - (1 << k) >= self.depth[v] {
                u = self.up[k][u];
            }
        }
        if u == v { return u; }
        for k in (0..self.log).rev() {
            if self.up[k][u] != self.up[k][v] {
                u = self.up[k][u];
                v = self.up[k][v];
            }
        }
        self.up[0][u]
    }
}

// ========== SPARSE TABLE FOR RMQ ==========
struct SparseTable {
    st: Vec<Vec<i64>>,
    log: Vec<usize>,
}

impl SparseTable {
    fn new(a: &[i64]) -> Self {
        let n = a.len();
        let logn = 64 - n.leading_zeros() as usize;
        let mut st = vec![vec![0; n]; logn];
        st[0].clone_from_slice(a);
        for k in 1..logn {
            for i in 0..=n - (1 << k) {
                st[k][i] = st[k - 1][i].min(st[k - 1][i + (1 << (k - 1))]);
            }
        }
        let mut log = vec![0; n + 1];
        for i in 2..=n {
            log[i] = log[i / 2] + 1;
        }
        Self { st, log }
    }

    fn query(&self, l: usize, r: usize) -> i64 {
        let k = self.log[r - l + 1];
        self.st[k][l].min(self.st[k][r - (1 << k) + 1])
    }
}

// ========== MO'S ALGORITHM ==========
struct Query {
    l: usize,
    r: usize,
    idx: usize,
}

fn mos_algorithm(n: usize, arr: &[usize], raw_queries: Vec<(usize, usize)>) -> Vec<usize> {
    let block_size = (n as f64).sqrt().ceil() as usize;
    let mut queries: Vec<_> = raw_queries.into_iter().enumerate().map(|(i, (l, r))| Query { l, r, idx: i }).collect();
    queries.sort_by_key(|q| (q.l / block_size, q.r));

    let mut res = vec![0; queries.len()];
    let mut freq = vec![0; 1_000_001];
    let mut curr_ans = 0;

    fn add(x: usize, freq: &mut [usize], curr: &mut usize) {
        *curr -= freq[x] * freq[x] * x;
        freq[x] += 1;
        *curr += freq[x] * freq[x] * x;
    }

    fn remove(x: usize, freq: &mut [usize], curr: &mut usize) {
        *curr -= freq[x] * freq[x] * x;
        freq[x] -= 1;
        *curr += freq[x] * freq[x] * x;
    }

    let (mut l, mut r) = (0, 0);
    add(arr[0], &mut freq, &mut curr_ans);
    for q in queries {
        while r < q.r { r += 1; add(arr[r], &mut freq, &mut curr_ans); }
        while r > q.r { remove(arr[r], &mut freq, &mut curr_ans); r -= 1; }
        while l < q.l { remove(arr[l], &mut freq, &mut curr_ans); l += 1; }
        while l > q.l { l -= 1; add(arr[l], &mut freq, &mut curr_ans); }
        res[q.idx] = curr_ans;
    }
    res
}

//use std::collections::HashMap;

// Count subarrays whose XOR is K
fn count_subarrays_with_xor(arr: &[i64], k: i64) -> i64 {
    let mut count = 0;
    let mut xor_map = HashMap::new();
    let mut prefix_xor = 0;
    xor_map.insert(0, 1);

    for &num in arr {
        prefix_xor ^= num;
        if let Some(&freq) = xor_map.get(&(prefix_xor ^ k)) {
            count += freq;
        }
        *xor_map.entry(prefix_xor).or_insert(0) += 1;
    }
    count
}

// Compute XOR basis for linear independence
struct XorBasis {
    basis: [i64; 64],
    sz: usize,
}

impl XorBasis {
    fn new() -> Self {
        Self { basis: [0; 64], sz: 0 }
    }

    fn insert(&mut self, mut x: i64) {
        for i in (0..64).rev() {
            if x & (1 << i) == 0 { continue; }
            if self.basis[i] == 0 {
                self.basis[i] = x;
                self.sz += 1;
                return;
            }
            x ^= self.basis[i];
        }
    }

    fn max_xor(&self) -> i64 {
        let mut res = 0;
        for &b in &self.basis {
            res = res.max(res ^ b);
        }
        res
    }

    fn count_subsets(&self) -> usize {
        1 << self.sz
    }
}

// Use in problems involving subsets of array with max XOR or independent vectors
fn example_xor_basis_problem() {
    let arr = vec![1, 2, 3, 4];
    let mut xb = XorBasis::new();
    for &x in &arr {
        xb.insert(x);
    }
    println!("Max XOR: {}, Independent subsets: {}", xb.max_xor(), xb.count_subsets());
}


// Deep-loop: emulate for loop using custom stepper
fn step_range<F: FnMut(usize)>(start: usize, end: usize, mut f: F) {
    let mut k = start;
    while k < end {
        f(k);
        k += 1;
    }
}

// Simulate `for i in (a..b).rev()` with deep fold
fn rev_fold<F: FnMut(usize)>(a: usize, b: usize, mut f: F) {
    let mut i = b;
    while i > a {
        i -= 1;
        f(i);
    }
}

// Conditional executor with map
fn exec_if<T, F: FnOnce() -> T>(flag: bool, action: F) -> Option<T> {
    if flag { Some(action()) } else { None }
}

// Unique scanner from input stream
fn scan_vec<T: std::str::FromStr>(s: &str) -> Vec<T> {
    s.split_whitespace().filter_map(|x| x.parse::<T>().ok()).collect()
}

// Chunk processor
fn process_chunks<T: Copy>(arr: &[T], k: usize) -> Vec<Vec<T>> {
    arr.chunks(k).map(|c| c.to_vec()).collect()
}

// Tuple-based merge walker
fn merge_zip(a: &[i32], b: &[i32]) -> Vec<(i32, i32)> {
    a.iter().cloned().zip(b.iter().cloned()).collect()
}

// Custom indexing map builder
fn index_mapper(data: &[i32]) -> std::collections::HashMap<i32, usize> {
    let mut map = std::collections::HashMap::new();
    for (idx, val) in data.iter().enumerate() {
        map.insert(*val, idx);
    }
    map
}

// Logic pattern to flatten nested loops into map+flat_map
fn cartesian_prod(a: &[i32], b: &[i32]) -> Vec<(i32, i32)> {
    a.iter()
     .flat_map(|&x| b.iter().map(move |&y| (x, y)))
     .collect()
}

// Closure-based in-place filter
fn inplace_filter(v: &mut Vec<i32>, cond: impl Fn(&i32) -> bool) {
    *v = v.iter().cloned().filter(|x| cond(x)).collect();
}



// Modulo algebra obfuscator
fn mod_diff(a: i64, b: i64, m: i64) -> i64 {
    ((a % m - b % m + m) % m + m) % m
}

// Lazy digit counter
fn lazy_digit_sum(mut x: u64) -> impl Iterator<Item = u64> {
    std::iter::from_fn(move || {
        if x == 0 { return None; }
        let res = x % 10;
        x /= 10;
        Some(res)
    })
}

// Reversible traversal pattern (xors direction)
fn traverse_grid(h: usize, w: usize, mut f: impl FnMut(usize, usize)) {
    for y in 0..h {
        let row: Box<dyn Iterator<Item = usize>> = if y & 1 == 0 {
            Box::new(0..w)
        } else {
            Box::new((0..w).rev())
        };
        for x in row {
            f(y, x);
        }
    }
}

// Obscure hashable pair set
fn pair_set(input: &[(i64, i64)]) -> std::collections::BTreeSet<(i64, i64)> {
    input.iter().map(|&(a, b)| (a ^ 31337, b ^ 7331)).collect()
}




/*fn join_combos<'a>(words: &[&str]) -> impl FnMut(usize) -> String {
    let mut cache: HashMap<usize, String> = HashMap::new(); // now properly mutable

    move |limit: usize| {
        if let Some(s) = cache.get(&limit) {
            return s.clone();
        }
        let s = words.iter().take(limit).cloned().collect::<Vec<_>>().join("-");
        cache.insert(limit, s.clone()); // cache is mutable now
        s
    }
}*/

/*use std::collections::HashMap;

fn join_combos<'a>(words: &[&'a str]) -> impl FnMut(usize) -> String + 'a {
    let mut cache: HashMap<usize, String> = HashMap::new(); // now properly mutable

    move |limit: usize| {
        if let Some(s) = cache.get(&limit) {
            return s.clone();
        }

        let s: String = words.iter().take(limit).cloned().collect::<Vec<_>>().join("-");

        cache.insert(limit, s.clone()); // cache is mutable now
        s
    }
}*/
fn join_combos<'a>(words: &'a [&'a str]) -> impl FnMut(usize) -> String + 'a {
    let mut cache: HashMap<usize, String> = HashMap::new(); // now properly mutable

    move |limit: usize| {
        if let Some(s) = cache.get(&limit) {
            return s.clone();
        }

        let s: String = words.iter().take(limit).cloned().collect::<Vec<_>>().join("-");

        cache.insert(limit, s.clone()); // cache is mutable now
        s
    }
}


// Inverted Index constructor
fn inverted_index(docs: &[&str]) -> HashMap<String, Vec<usize>> {
    let mut index: HashMap<String, Vec<usize>> = HashMap::new(); // Explicit type added
    for (i, doc) in docs.iter().enumerate() {
        for word in doc.split_whitespace() {
            index.entry(word.to_string()).or_default().push(i);
        }
    }
    index
}


// Rare scan-transform-count with map
fn scan_binary_stats(bits: &str) -> (usize, usize) {
    bits.chars()
        .map(|c| if c == '1' { 1 } else { 0 })
        .fold((0, 0), |(ones, zeros), x| {
            if x == 1 { (ones + 1, zeros) } else { (ones, zeros + 1) }
        })
}

// Range executor with early stop
fn until_match<F: Fn(usize) -> bool>(start: usize, f: F) -> Option<usize> {
    (start..).take_while(|&x| !f(x)).last().map(|x| x + 1)
}

// Rare rolling XOR accumulator
fn xor_accumulate(data: &[u32]) -> Vec<u32> {
    let mut acc = 0;
    data.iter().map(|&x| {
        acc ^= x;
        acc
    }).collect()
}

// Triplet matcher with logic transform
fn match_triplets(input: &[i32]) -> Vec<(i32, i32, i32)> {
    let mut res = Vec::new();
    for i in 1..input.len() - 1 {
        if input[i] > input[i - 1] && input[i] > input[i + 1] {
            res.push((input[i - 1], input[i], input[i + 1]));
        }
    }
    res
}


// Generic parser for a single value
fn parse_val<T: FromStr>() -> T 
where
    <T as FromStr>::Err: Debug,
{
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().parse::<T>().unwrap()
}

// Parser for a vector of values
fn parse_vec<T: FromStr>() -> Vec<T> 
where
    <T as FromStr>::Err: Debug,
{
    let mut line = String::new();
    io::stdin().lock().read_line(&mut line).unwrap();
    line.trim().split_whitespace().map(|x| x.parse::<T>().unwrap()).collect()
}







/*-------------main function-------------*/

//use std::io::{self, BufRead, Write};


fn parse_line<T: std::str::FromStr>() -> Vec<T> {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    input_line
        .split_whitespace()
        .filter_map(|token| token.parse::<T>().ok())
        .collect()
}
//use std::io::{self, BufRead};

/*fn bin_search_r(n: i64, need: i128) -> i64 {
    if need == 0 {
        return 1;
    }
    let mut left = 1;
    let mut right = n;
    while left < right {
        let mid = (left + right) / 2;
        let edges = (mid as i128 * (mid as i128 - 1)) / 2;
        if edges >= need {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    left
}

fn process_single_case(n: i64, m: i64) -> i64 {
    let full_edge_count = (n as i128 * (n as i128 - 1)) / 2;
    let spare = full_edge_count - m as i128;

    let opt = ((n - 1) as i128 * (n - 2) as i128) / 2;
    let min_weight = (m as i128 - opt).max(0);

    let tight_cluster = bin_search_r(n, spare);
    let max_weight = (n - tight_cluster) as i128;

    if max_weight < min_weight {
        return 0;
    }

    let total_choices = max_weight - min_weight + 1;
    let total_sum = (min_weight + max_weight) * total_choices / 2;
    total_sum as i64
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let t = lines.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    for _ in 0..t {
        if let Some(Ok(line)) = lines.next() {
            let parts: Vec<i64> = line
                .split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect();
            let result = process_single_case(parts[0], parts[1]);
            println!("{}", result);
        }
    }
}
*/
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

struct Node {
    val: i64,
    id: usize,
    alive: bool,
    prev: Option<Weak<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: i64, id: usize) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            val,
            id,
            alive: true,
            prev: None,
            next: None,
        }))
    }
}

struct Pair {
    sum: i64,
    leftid: usize,
    left: Rc<RefCell<Node>>,
    right: Rc<RefCell<Node>>,
}

impl Pair {
    fn new(s: i64, id: usize, l: Rc<RefCell<Node>>, r: Rc<RefCell<Node>>) -> Self {
        Pair { sum: s, leftid: id, left: l, right: r }
    }
}

impl Eq for Pair {}
impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.sum == other.sum && self.leftid == other.leftid
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.sum == other.sum {
            other.leftid.cmp(&self.leftid)
        } else {
            other.sum.cmp(&self.sum)
        }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

//pub struct Solution;

impl Solution {
    pub fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut nodes: Vec<Rc<RefCell<Node>>> = Vec::new();
        for i in 0..n {
            nodes.push(Node::new(nums[i] as i64, i));
        }

        for i in 0..n - 1 {
            nodes[i].borrow_mut().next = Some(nodes[i + 1].clone());
            nodes[i + 1].borrow_mut().prev = Some(Rc::downgrade(&nodes[i]));
        }

        let mut head = nodes[0].clone();
        let mut violation = 0;

        {
            let mut cur = Some(head.clone());
            while let Some(node) = cur {
                let next = node.borrow().next.clone();
                if let Some(ref nxt) = next {
                    if node.borrow().val > nxt.borrow().val {
                        violation += 1;
                    }
                }
                cur = next;
            }
        }

        if violation == 0 {
            return 0;
        }

        let mut pq = BinaryHeap::new();
        {
            let mut cur = Some(head.clone());
            while let Some(node) = cur {
                let next = node.borrow().next.clone();
                if let Some(ref nxt) = next {
                    pq.push(Pair::new(node.borrow().val + nxt.borrow().val, node.borrow().id, node.clone(), nxt.clone()));
                }
                cur = next;
            }
        }

        let mut ops = 0;
        while violation > 0 {
            while let Some(cur) = pq.pop() {
                if !cur.left.borrow().alive || !cur.right.borrow().alive {
                    continue;
                }

                let next_ptr_match = cur.left.borrow().next.as_ref().map(|n| Rc::ptr_eq(n, &cur.right)).unwrap_or(false);
                if !next_ptr_match {
                    continue;
                }

                let left = cur.left.clone();
                let right = cur.right.clone();
                let new_val = left.borrow().val + right.borrow().val;

                let ln = left.borrow().prev.as_ref().and_then(|w| w.upgrade());
                let rn = right.borrow().next.clone();

                let mut old_count = 0;
                if let Some(ref l) = ln {
                    if l.borrow().val > left.borrow().val {
                        old_count += 1;
                    }
                }
                if left.borrow().val > right.borrow().val {
                    old_count += 1;
                }
                if let Some(ref r) = rn {
                    if right.borrow().val > r.borrow().val {
                        old_count += 1;
                    }
                }

                let mut new_count = 0;
                if let Some(ref l) = ln {
                    if l.borrow().val > new_val {
                        new_count += 1;
                    }
                }
                if let Some(ref r) = rn {
                    if new_val > r.borrow().val {
                        new_count += 1;
                    }
                }

                violation = violation - old_count + new_count;

                let new_node = Node::new(new_val, left.borrow().id);

                if let Some(ref l) = ln {
                    l.borrow_mut().next = Some(new_node.clone());
                    new_node.borrow_mut().prev = Some(Rc::downgrade(l));
                } else {
                    head = new_node.clone();
                }

                if let Some(ref r) = rn {
                    r.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                    new_node.borrow_mut().next = Some(r.clone());
                }

                left.borrow_mut().alive = false;
                right.borrow_mut().alive = false;

                if let Some(ref l) = ln {
                    pq.push(Pair::new(l.borrow().val + new_node.borrow().val, l.borrow().id, l.clone(), new_node.clone()));
                }
                if let Some(ref r) = rn {
                    pq.push(Pair::new(new_node.borrow().val + r.borrow().val, new_node.borrow().id, new_node.clone(), r.clone()));
                }

                ops += 1;
                break;
            }
        }

        ops
    }
}


