/*impl Solution {
    pub fn longest_palindrome(s: String, t: String) -> i32 {
        
    }
}*/
 impl Solution {
    pub fn longest_palindrome(s: String, t: String) -> i32 {
        let mut max_len = 1;
        
        let check_palindrome = |s: &str| -> bool {
            s.chars().eq(s.chars().rev())
        };
        
        for i in 0..s.len() {
            for j in i..s.len() {
                let left_part = &s[i..=j];
                for k in 0..t.len() {
                    for l in k..t.len() {
                        let right_part = &t[k..=l];
                        let candidate = format!("{}{}", left_part, right_part);
                        if check_palindrome(&candidate) {
                            max_len = max_len.max(candidate.len() as i32);
                        }
                    }
                }
            }
        }
        
        for i in 0..t.len() {
            for j in i..t.len() {
                let sub = &t[i..=j];
                if check_palindrome(sub) {
                    max_len = max_len.max(sub.len() as i32);
                }
            }
        }
        
        for i in 0..s.len() {
            for j in i..s.len() {
                let sub = &s[i..=j];
                if check_palindrome(sub) {
                    max_len = max_len.max(sub.len() as i32);
                }
            }
        }
        
        max_len
    }
}
