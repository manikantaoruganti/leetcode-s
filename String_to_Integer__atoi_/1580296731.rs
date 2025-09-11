impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        s.trim()
            .chars()
            .scan((0i32, 1, false), |(num, sign, started), c| {
                if !*started {
                    if c == '-' { *sign = -1; *started = true; }
                    else if c == '+' { *started = true; }
                    else if c.is_digit(10) { *num = c.to_digit(10).unwrap() as i32; *started = true; }
                    else { return None; }
                } else if c.is_digit(10) {
                    *num = num.saturating_mul(10).saturating_add(c.to_digit(10).unwrap() as i32 * *sign);
                } else { return None; }
                Some(*num)
            })
            .last()
            .unwrap_or(0)
    }
}

 