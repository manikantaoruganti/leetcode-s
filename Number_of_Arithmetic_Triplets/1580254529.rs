
use std::collections::HashSet;

impl Solution {
    pub fn arithmetic_triplets(nums: Vec<i32>, diff: i32) -> i32 {
        let mut set = HashSet::new();
        let mut count = 0;

        for &num in &nums {
            if set.contains(&(num - diff)) && set.contains(&(num - 2 * diff)) {
                count += 1;
            }
            set.insert(num);
        }

        count
    }
}
