
impl Solution {
    pub fn longest_palindrome(s: String, t: String) -> i32 {
        let n = s.len();
        let m = t.len();
        let s_chars: Vec<char> = s.chars().collect();
        let t_chars: Vec<char> = t.chars().collect();

        let mut pal_s = vec![vec![false; n]; n];
        let mut pal_t = vec![vec![false; m]; m];
        
        for i in 0..n {
            pal_s[i][i] = true;
        }
        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                if s_chars[i] == s_chars[j] && (len == 2 || pal_s[i + 1][j - 1]) {
                    pal_s[i][j] = true;
                }
            }
        }
        for i in 0..m {
            pal_t[i][i] = true;
        }
        for len in 2..=m {
            for i in 0..=m - len {
                let j = i + len - 1;
                if t_chars[i] == t_chars[j] && (len == 2 || pal_t[i + 1][j - 1]) {
                    pal_t[i][j] = true;
                }
            }
        }
        
        let mut lps_start = vec![0; n];
        for i in 0..n {
            for j in i..n {
                if pal_s[i][j] {
                    lps_start[i] = lps_start[i].max(j - i + 1);
                }
            }
        }
        
        let mut lps_end = vec![0; m];
        for j in 0..m {
            for i in 0..=j {
                if pal_t[i][j] {
                    lps_end[j] = lps_end[j].max(j - i + 1);
                }
            }
        }
        
        let mut ans = *lps_start.iter().max().unwrap().max(lps_end.iter().max().unwrap());
        
        let mut r_t = t_chars.clone();
        r_t.reverse();
        
        let mut dp = vec![vec![0; m]; n];
        for i in 0..n {
            for j in 0..m {
                if s_chars[i] == r_t[j] {
                    dp[i][j] = if i == 0 || j == 0 { 1 } else { dp[i - 1][j - 1] + 1 };
                    let l = dp[i][j];
                    let start_s = i + 1 - l;
                    let t_start = m - 1 - j;
                    if t_start < 0 { continue; }
                    let candidate1 = 2 * l + if start_s + l < n { lps_start[start_s + l] } else { 0 };
                    let candidate2 = 2 * l + if t_start > 0 { lps_end[t_start - 1] } else { 0 };
                    ans = ans.max(candidate1.max(candidate2));
                }
            }
        }
        ans as i32
    }
}