/*impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        
    }
}*/
 impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        let n = s.len();
        let mut ones_count = s.chars().filter(|&c| c == '1').count() as i32;
        let mut max_active = ones_count;

        // Augment `s` as if surrounded by '1's
        let t = format!("1{}1", s);
        let t_chars: Vec<char> = t.chars().collect();
        let m = t.len();

        let mut i = 1;
        while i < m - 1 {
            if t_chars[i] == '1' {
                let mut j = i;
                
                // Move j to the end of contiguous '1' block
                while j < m && t_chars[j] == '1' {
                    j += 1;
                }
                
                // Check if a valid trade can be made
                if j < m - 1 && t_chars[i - 1] == '0' && t_chars[j] == '0' {
                    let mut left_zeros = 0;
                    let mut right_zeros = 0;
                    
                    // Count zeros to the left
                    let mut k = i - 1;
                    while k >= 0 && t_chars[k] == '0' {
                        left_zeros += 1;
                        if k == 0 { break; }  // Avoid underflow
                        k -= 1;
                    }
                    
                    // Count zeros to the right
                    let mut k = j;
                    while k < m && t_chars[k] == '0' {
                        right_zeros += 1;
                        k += 1;
                    }

                    // Update the maximum active sections
                    max_active = max_active.max(ones_count + left_zeros + right_zeros);
                }
                
                i = j;
            } else {
                i += 1;
            }
        }

        max_active.min(n as i32) // Ensuring the final count doesn't exceed n
    }
}
