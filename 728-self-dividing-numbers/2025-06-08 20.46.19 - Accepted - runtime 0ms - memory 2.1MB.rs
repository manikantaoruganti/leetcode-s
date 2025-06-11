impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut result = Vec::new();

        for num in left..=right {
            if Self::is_self_dividing(num) {
                result.push(num);
            }
        }

        result
    }

    fn is_self_dividing(mut num: i32) -> bool {
        let original = num;
        while num > 0 {
            let digit = num % 10;
            if digit == 0 || (original % digit) != 0 {
                return false;
            }
            num /= 10;
        }
        true
    }
}

