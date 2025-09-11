impl Solution {
    pub fn number_of_lines(widths: Vec<i32>, s: String) -> Vec<i32> {
       let mut lines = 1;
let mut width = 0;

for c in s.chars() {
    let w = widths[c as usize - 'a' as usize];
    if width + w > 100 {
        lines += 1;
        width = w;
    } else {
        width += w;
    }
}

vec![lines, width]
        
    }
}