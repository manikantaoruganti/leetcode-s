 impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        s.chars().enumerate()
            .map(|(i, c)| (26 - (c as u8 - b'a') as i32) * (i as i32 + 1))
            .sum()
    }
}
