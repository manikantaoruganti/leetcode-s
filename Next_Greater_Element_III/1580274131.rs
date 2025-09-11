impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut digits: Vec<char> = n.to_string().chars().collect();
        let len = digits.len();

        let mut i = len as isize - 2;
        while i >= 0 && digits[i as usize] >= digits[i as usize + 1] {
            i -= 1;
        }

        if i < 0 {
            return -1;
        }

        let mut j = len as isize - 1;
        while digits[j as usize] <= digits[i as usize] {
            j -= 1;
        }

        digits.swap(i as usize, j as usize);
        digits[i as usize + 1..].reverse();

        let next_number = digits.iter().collect::<String>().parse::<i32>().unwrap_or(-1);
        if next_number > n { next_number } else { -1 }
    }
}
