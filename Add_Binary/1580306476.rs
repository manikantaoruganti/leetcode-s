impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result = String::new();
        let (mut i, mut j, mut carry) = (a.len() as isize - 1, b.len() as isize - 1, 0);

        while i >= 0 || j >= 0 || carry > 0 {
            let bit_a = if i >= 0 { a.as_bytes()[i as usize] - b'0' } else { 0 };
            let bit_b = if j >= 0 { b.as_bytes()[j as usize] - b'0' } else { 0 };

            let sum = bit_a + bit_b + carry;
            result.push((sum % 2 + b'0') as char);
            carry = sum / 2;

            i -= 1;
            j -= 1;
        }

        result.chars().rev().collect()
    }
}
