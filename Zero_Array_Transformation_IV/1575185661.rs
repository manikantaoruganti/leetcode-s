use std::collections::VecDeque;

impl Solution {
    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let max_value = 1000; // Max possible value in nums
        let mut bit_dp = vec![VecDeque::from(vec![false; max_value + 1]); n];

        // Initialize DP bitset with 1 at position 0
        for j in 0..n {
            bit_dp[j][0] = true;
        }

        // Check if nums is already zero array
        if nums.iter().all(|&x| x == 0) {
            return 0;
        }

        // Process queries
        for (query_idx, query) in queries.iter().enumerate() {
            let (left_idx, right_idx, value) = (query[0] as usize, query[1] as usize, query[2] as usize);

            for j in left_idx..=right_idx {
                for pos in (value..=max_value).rev() {
                    if bit_dp[j][pos - value] {
                        bit_dp[j][pos] = true;
                    }
                }
            }

            // Check if all numbers can be transformed to zero
            let mut all_reached = true;
            for j in 0..n {
                if !bit_dp[j][nums[j] as usize] {
                    all_reached = false;
                    break;
                }
            }

            if all_reached {
                return (query_idx + 1) as i32;
            }
        }

        -1
    }
}
