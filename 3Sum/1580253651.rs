impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        nums.sort(); // Sort the array to use the two-pointer technique
        
        let n = nums.len();
        for i in 0..n {
            if i > 0 && nums[i] == nums[i - 1] {
                continue; // Skip duplicate elements
            }

            let (mut left, mut right) = (i + 1, n - 1);
            while left < right {
                let sum = nums[i] + nums[left] + nums[right];
                
                if sum == 0 {
                    res.push(vec![nums[i], nums[left], nums[right]]);
                    
                    // Move left pointer forward and avoid duplicates
                    while left < right && nums[left] == nums[left + 1] {
                        left += 1;
                    }
                    // Move right pointer backward and avoid duplicates
                    while left < right && nums[right] == nums[right - 1] {
                        right -= 1;
                    }
                    
                    left += 1;
                    right -= 1;
                } else if sum < 0 {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
        
        res
    }
}

