impl Solution {
    pub fn unequal_triplets(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut freq = std::collections::HashMap::new();
        
        // Count occurrences of each number
        for &num in &nums {
            *freq.entry(num).or_insert(0) += 1;
        }
        
        let mut left = 0;
        let mut right = nums.len() as i32;
        
        for (&num, &count_num) in &freq {
            right -= count_num; // Remaining elements after the current number
            count += left * count_num * right;
            left += count_num; // Move count to the left for future elements
        }
        
        count
    }
}
