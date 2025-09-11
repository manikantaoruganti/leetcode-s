impl Solution {
    pub fn sort_sentence(s: String) -> String {
        let mut words = vec![""; 9];
        let mut count = 0;
        
        for word in s.split_whitespace() {
            let idx = word.chars().last().unwrap().to_digit(10).unwrap() as usize - 1;
            words[idx] = &word[..word.len() - 1];
            count += 1;
        }
        
        words[..count].join(" ")
    }
}
