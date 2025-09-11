use std::collections::HashSet;

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut has_non_negative = false;
        let mut pos_sum = 0;
        let mut pos_set = HashSet::new();
        let mut max_neg = -101;

        for &x in &nums {
            if x >= 0 {
                has_non_negative = true;
                if x > 0 {
                    pos_set.insert(x);
                }
            } else {
                max_neg = max_neg.max(x);
            }
        }

        if has_non_negative {
            pos_sum = pos_set.iter().sum();
            return pos_sum;
        }

        max_neg
    }
}
