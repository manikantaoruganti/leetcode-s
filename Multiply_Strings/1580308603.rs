impl Solution {
    pub fn multiply(num1: String, num2: String) -> String {
        if num1 == "0" || num2 == "0" {
            return "0".to_string();
        }

        let mut result = vec![0; num1.len() + num2.len()];
        let num1 = num1.chars().rev().collect::<Vec<char>>();
        let num2 = num2.chars().rev().collect::<Vec<char>>();

        for (i, &c1) in num1.iter().enumerate() {
            for (j, &c2) in num2.iter().enumerate() {
                let mul = (c1 as u8 - b'0') as usize * (c2 as u8 - b'0') as usize;
                let sum = mul + result[i + j];
                result[i + j] = sum % 10;
                result[i + j + 1] += sum / 10;
            }
        }

        while result.len() > 1 && result.last() == Some(&0) {
            result.pop();
        }

        result.into_iter().rev().map(|d| (d as u8 + b'0') as char).collect()
    }
}
