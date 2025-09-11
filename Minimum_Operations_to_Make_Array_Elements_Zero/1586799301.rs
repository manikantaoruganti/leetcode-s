impl Solution {
    pub fn min_operations(queries: Vec<Vec<i32>>) -> i64 {
        let mut total_operations = 0i64;
        
        for query in queries {
            let l = query[0] as i64;
            let r = query[1] as i64;
            
            let mut sum_steps = 0i64;
            let mut max_steps = 0i64;
            
            let mut k = 1i64;
            loop {
                let lower = 4i64.pow((k - 1) as u32);
                let upper = 4i64.pow(k as u32) - 1;
                
                if lower > r {
                    break;
                }
                
                if upper < l {
                    k += 1;
                    continue;
                }
                
                let current_l = std::cmp::max(l, lower);
                let current_r = std::cmp::min(r, upper);
                let count = current_r - current_l + 1;
                sum_steps += count * k;
                
                if current_r >= lower {
                    max_steps = k;
                }
                
                k += 1;
            }
            
            let operations = std::cmp::max(max_steps, (sum_steps + 1) / 2);
            total_operations += operations;
        }
        
        total_operations
    }
}