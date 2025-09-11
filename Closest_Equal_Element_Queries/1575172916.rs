use std::collections::HashMap;

impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut positions: HashMap<i32, Vec<usize>> = HashMap::new();

        // Store indices of each number
        for (i, &num) in nums.iter().enumerate() {
            positions.entry(num).or_insert(Vec::new()).push(i);
        }

        let mut ans = Vec::with_capacity(queries.len());

        for &q in &queries {
            let val = nums[q as usize];
            if let Some(idx_list) = positions.get(&val) {
                if idx_list.len() < 2 {
                    ans.push(-1);
                    continue;
                }

                // Binary search to find closest position
                if let Ok(pos) = idx_list.binary_search(&(q as usize)) {
                    let left_idx = if pos == 0 { idx_list.len() - 1 } else { pos - 1 };
                    let right_idx = (pos + 1) % idx_list.len();

                    let left_candidate = idx_list[left_idx];
                    let right_candidate = idx_list[right_idx];

                    let dist_left = ((q as isize - left_candidate as isize).abs()).min(n as isize - (q as isize - left_candidate as isize).abs()) as i32;
                    let dist_right = ((q as isize - right_candidate as isize).abs()).min(n as isize - (q as isize - right_candidate as isize).abs()) as i32;

                    ans.push(dist_left.min(dist_right));
                } else {
                    ans.push(-1);
                }
            } else {
                ans.push(-1);
            }
        }

        ans
    }
}
