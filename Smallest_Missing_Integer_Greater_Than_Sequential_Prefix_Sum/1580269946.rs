use std::collections::HashSet;

impl Solution {
    pub fn missing_integer(nums: Vec<i32>) -> i32 {
        let mut sum = nums[0];
        let mut expected = nums[0];

        for &num in nums.iter().skip(1) {
            if num == expected + 1 {
                sum += num;
                expected = num;
            } else {
                break;
            }
        }

        let num_set: HashSet<i32> = nums.into_iter().collect();
        let mut missing = sum;

        while num_set.contains(&missing) {
            missing += 1;
        }

        missing
    }
}
