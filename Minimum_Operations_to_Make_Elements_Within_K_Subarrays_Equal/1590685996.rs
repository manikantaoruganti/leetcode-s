/*impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
        
    }
}*/
  use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

struct SlidingMedian {
    left: BinaryHeap<i32>,
    right: BinaryHeap<Reverse<i32>>,
    delayed: HashMap<i32, usize>,
    sum_left: i64,
    sum_right: i64,
    size_left: usize,
    size_right: usize,
}

impl SlidingMedian {
    fn new() -> Self {
        Self {
            left: BinaryHeap::new(),
            right: BinaryHeap::new(),
            delayed: HashMap::new(),
            sum_left: 0,
            sum_right: 0,
            size_left: 0,
            size_right: 0,
        }
    }
    
    fn add(&mut self, num: i32) {
        if self.size_left == 0 || num <= *self.left.peek().unwrap() {
            self.left.push(num);
            self.sum_left += num as i64;
            self.size_left += 1;
        } else {
            self.right.push(Reverse(num));
            self.sum_right += num as i64;
            self.size_right += 1;
        }
        self.balance();
    }
    
    fn remove(&mut self, num: i32) {
        *self.delayed.entry(num).or_insert(0) += 1;
        if !self.left.is_empty() && num <= *self.left.peek().unwrap() {
            self.sum_left -= num as i64;
            self.size_left -= 1;
        } else {
            self.sum_right -= num as i64;
            self.size_right -= 1;
        }
        self.balance();
    }
    
    fn balance(&mut self) {
        if self.size_left > self.size_right + 1 {
            let top = self.left.pop().unwrap();
            self.sum_left -= top as i64;
            self.size_left -= 1;
            self.right.push(Reverse(top));
            self.sum_right += top as i64;
            self.size_right += 1;
        } else if self.size_left < self.size_right {
            let top = self.right.pop().unwrap().0;
            self.sum_right -= top as i64;
            self.size_right -= 1;
            self.left.push(top);
            self.sum_left += top as i64;
            self.size_left += 1;
        }
        self.prune_left();
        self.prune_right();
    }
    
    fn prune_left(&mut self) {
        while let Some(&num) = self.left.peek() {
            if let Some(&count) = self.delayed.get(&num) {
                if count > 0 {
                    self.left.pop();
                    if count == 1 {
                        self.delayed.remove(&num);
                    } else {
                        self.delayed.insert(num, count - 1);
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    
    fn prune_right(&mut self) {
        while let Some(&Reverse(num)) = self.right.peek() {
            if let Some(&count) = self.delayed.get(&num) {
                if count > 0 {
                    self.right.pop();
                    if count == 1 {
                        self.delayed.remove(&num);
                    } else {
                        self.delayed.insert(num, count - 1);
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    
    fn get_median(&self) -> i32 {
        *self.left.peek().unwrap()
    }
    
    fn get_cost(&self) -> i64 {
        let median = self.get_median() as i64;
        median * self.size_left as i64 - self.sum_left + self.sum_right - median * self.size_right as i64
    }
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
        let n = nums.len();
        let w = (n - x as usize + 1) as usize;
        let inf = i64::MAX / 2;
        let mut cost = vec![0; w];
        let mut sm = SlidingMedian::new();
        
        for &num in &nums[..x as usize] {
            sm.add(num);
        }
        cost[0] = sm.get_cost();
        
        for i in 1..w {
            sm.remove(nums[i - 1]);
            sm.add(nums[i + x as usize - 1]);
            cost[i] = sm.get_cost();
        }
        
        let mut dp = vec![vec![inf; (k + 1) as usize]; w];
        for i in 0..w {
            dp[i][0] = 0;
        }
        
        for i in 0..w {
            for j in 1..=k as usize {
                if i > 0 {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j]);
                }
                let prev = i as isize - x as isize;
                if prev >= 0 {
                    dp[i][j] = dp[i][j].min(dp[prev as usize][j - 1] + cost[i]);
                } else if j == 1 {
                    dp[i][j] = dp[i][j].min(cost[i]);
                }
            }
        }
        dp[w - 1][k as usize]
    }
}

