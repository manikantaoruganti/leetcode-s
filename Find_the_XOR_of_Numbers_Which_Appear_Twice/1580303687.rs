use std::collections::HashSet;

impl Solution {
    pub fn duplicate_numbers_xor(nums: Vec<i32>) -> i32 {
        nums.iter().fold((0, HashSet::new()), |(xor, mut seen), &x| {
            if !seen.insert(x) { (xor ^ x, seen) } else { (xor, seen) }
        }).0
    }
}
