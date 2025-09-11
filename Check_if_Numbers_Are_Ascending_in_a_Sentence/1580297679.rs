impl Solution {
    pub fn are_numbers_ascending(s: String) -> bool {
        let mut prev = 0;
        s.split_whitespace()
            .filter_map(|word| word.parse::<i32>().ok())
            .all(|num| { 
                let res = num > prev; 
                prev = num; 
                res 
            })
    }
}
