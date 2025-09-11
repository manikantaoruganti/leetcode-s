impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let n = s.len();
let chars: Vec<char> = s.chars().collect();
let mut res = vec![n as i32; n];
let mut prev = n as i32;

// Left to right pass
for i in 0..n {
    if chars[i] == c {
        prev = i as i32;
    }
    res[i] = (i as i32 - prev).abs();
}

// Right to left pass
prev = 2 * n as i32;
for i in (0..n).rev() {
    if chars[i] == c {
        prev = i as i32;
    }
    res[i] = res[i].min((i as i32 - prev).abs());
}

res
        
    }
}