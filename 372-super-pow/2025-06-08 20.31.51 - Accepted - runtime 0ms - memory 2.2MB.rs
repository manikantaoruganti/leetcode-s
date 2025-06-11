impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        const MOD: i32 = 1337;

        fn mod_pow(mut x: i32, mut n: i32) -> i32 {
            let mut result = 1;
            x %= MOD;
            while n > 0 {
                if n % 2 == 1 {
                    result = (result * x) % MOD;
                }
                x = (x * x) % MOD;
                n /= 2;
            }
            result
        }

        fn helper(a: i32, b: &[i32]) -> i32 {
            if b.is_empty() {
                return 1;
            }
            let last = *b.last().unwrap();
            let part = helper(a, &b[..b.len() - 1]);
            (mod_pow(part, 10) * mod_pow(a, last)) % MOD
        }

        helper(a, &b)
    }
}
