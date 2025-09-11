impl Solution {
    pub fn count_quadruplets(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut count = 0;

        for a in 0..n {
            for b in (a + 1)..n {
                for c in (b + 1)..n {
                    for d in (c + 1)..n {
                        if nums[a] + nums[b] + nums[c] == nums[d] {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }
}
