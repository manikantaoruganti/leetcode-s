use std::collections::HashMap;
use std::collections::VecDeque;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut next_greater = HashMap::new();
        let mut stack = VecDeque::new();

        for &num in nums2.iter() {
            while let Some(&top) = stack.back() {
                if top < num {
                    next_greater.insert(stack.pop_back().unwrap(), num);
                } else {
                    break;
                }
            }
            stack.push_back(num);
        }

        nums1.iter().map(|&num| *next_greater.get(&num).unwrap_or(&-1)).collect()
    }
}
