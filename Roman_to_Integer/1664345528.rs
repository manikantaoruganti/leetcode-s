 use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut roman_map = HashMap::new();
        roman_map.insert('I', 1);
        roman_map.insert('V', 5);
        roman_map.insert('X', 10);
        roman_map.insert('L', 50);
        roman_map.insert('C', 100);
        roman_map.insert('D', 500);
        roman_map.insert('M', 1000);

        let chars: Vec<char> = s.chars().collect();
        let mut total = 0;
        let mut prev = 0;

        for &ch in chars.iter().rev() {
            let value = *roman_map.get(&ch).unwrap();
            if value < prev {
                total -= value;
            } else {
                total += value;
            }
            prev = value;
        }

        total
    }
}
