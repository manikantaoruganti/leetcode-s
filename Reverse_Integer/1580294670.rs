impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut num = x;
        let mut rev: i32 = 0;

        while num != 0 {
            if let Some(new_rev) = rev.checked_mul(10).and_then(|r| r.checked_add(num % 10)) {
                rev = new_rev;
            } else {
                return 0;
            }
            num /= 10;
        }

        rev
    }
}
