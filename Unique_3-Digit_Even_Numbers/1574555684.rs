  use std::collections::HashSet;

impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let mut unique_numbers = HashSet::new();
        let n = digits.len();

        for i in 0..n {
            if digits[i] == 0 { continue; } // First digit cannot be 0

            for j in 0..n {
                if i == j { continue; } // Unique indices

                for k in 0..n {
                    if i == k || j == k { continue; } // Unique indices
                    if digits[k] % 2 != 0 { continue; } // Last digit must be even

                    let num = digits[i] * 100 + digits[j] * 10 + digits[k];
                    unique_numbers.insert(num);
                }
            }
        }
        unique_numbers.len() as i32
    }
}
