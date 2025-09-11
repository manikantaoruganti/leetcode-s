/*impl Solution {
    pub fn max_product(nums: Vec<i32>, k: i32, limit: i32) -> i32 {
        
    }
}*/
  use std::collections::{HashMap, HashSet};

//pub struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>, k: i32, limit: i32) -> i32 {
        let inf = limit + 1;
        let mut dp: HashMap<(i32, i32), HashSet<i32>> = HashMap::new();

        for &x in nums.iter() {
            let mut new_dp = dp.clone();

            // Start a new subsequence with just x
            if x == 0 {
                new_dp.entry((1, 0)).or_default().insert(0);
            } else if x <= limit {
                new_dp.entry((1, x)).or_default().insert(x);
            } else {
                new_dp.entry((1, inf)).or_default().insert(x);
            }

            for ((parity, prod), set) in dp.iter() {
                for &alt in set {
                    let new_parity = 1 - parity;
                    let new_alt = if *parity == 0 { alt + x } else { alt - x };

                    let new_prod = if x == 0 {
                        0
                    } else if *prod == inf {
                        inf
                    } else {
                        let candidate = *prod as i64 * x as i64;
                        if candidate <= limit as i64 {
                            candidate as i32
                        } else {
                            inf
                        }
                    };

                    new_dp.entry((new_parity, new_prod)).or_default().insert(new_alt);
                }
            }

            dp = new_dp;
        }

        // Extract the maximum valid product for alternating sum == k
        let mut best = -1;
        for ((_, prod), alts) in dp.iter() {
            if *prod <= limit && alts.contains(&k) {
                best = best.max(*prod);
            }
        }

        best
    }
}
