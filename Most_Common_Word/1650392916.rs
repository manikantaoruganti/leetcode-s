impl Solution {
    pub fn most_common_word(paragraph: String, banned: Vec<String>) -> String {
      use std::collections::{HashMap, HashSet};

let banned_set: HashSet<_> = banned.iter().map(|w| w.to_string()).collect();
let mut freq: HashMap<String, i32> = HashMap::new();

let clean = paragraph
    .to_lowercase()
    .chars()
    .map(|c| if c.is_ascii_alphabetic() { c } else { ' ' })
    .collect::<String>();

for word in clean.split_whitespace() {
    if !banned_set.contains(word) {
        *freq.entry(word.to_string()).or_insert(0) += 1;
    }
}

freq.into_iter()
    .max_by_key(|&(_, count)| count)
    .unwrap()
    .0
        
    }
}