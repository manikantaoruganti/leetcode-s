impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let mut n = x;
        let mut res = 0;
        for _ in 0..32 {
            res = (res << 1) | (n & 1);
            n >>= 1;
        }
        res
    }
}
