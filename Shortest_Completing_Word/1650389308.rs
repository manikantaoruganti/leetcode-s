use std::collections::HashMap;

fn char_count(s: &str) -> [u8; 26] {
    let mut count = [0; 26];
    for c in s.chars().filter(|c| c.is_alphabetic()) {
        count[c.to_ascii_lowercase() as usize - 'a' as usize] += 1;
    }
    count
}

impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let target = char_count(&license_plate);
        words.into_iter()
            .filter(|word| {
                let word_count = char_count(word);
                (0..26).all(|i| word_count[i] >= target[i])
            })
            .min_by_key(|w| w.len())
            .unwrap()
    }
}

