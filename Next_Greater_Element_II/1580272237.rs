impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![-1; n];
        let mut stack = Vec::new();

        for i in 0..(2 * n) {
            let num = nums[i % n];
            while let Some(&idx) = stack.last() {
                if nums[idx] < num {
                    result[idx] = num;
                    stack.pop();
                } else {
                    break;
                }
            }
            if i < n {
                stack.push(i);
            }
        }

        result
    }
}
