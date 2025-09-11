impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let (mut left, mut right) = (0, nums.len() - 1);
        let mut index = nums.len();

        while index > 0 {
            index -= 1;
            if nums[left].abs() > nums[right].abs() {
                result[index] = nums[left] * nums[left];
                left += 1;
            } else {
                result[index] = nums[right] * nums[right];
                if right > 0 {
                    right -= 1;
                }
            }
        }

        result
    }
}
