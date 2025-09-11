use std::collections::HashMap;

struct BeautifulNumbersSolver {
    s: Vec<char>,
    dp: HashMap<(usize, bool, bool, bool, i32, i32, i32, i32, i32), i64>,
}

impl BeautifulNumbersSolver {
    fn rec(
        &mut self,
        pos: usize,
        tight: bool,
        started: bool,
        has_zero: bool,
        sum: i32,
        e2: i32,
        e3: i32,
        e5: i32,
        e7: i32,
    ) -> i64 {
        if pos == self.s.len() {
            if !started {
                return 0;
            }
            if has_zero {
                return 1;
            }

            let mut prod = 1;
            for _ in 0..e2 {
                prod *= 2;
            }
            for _ in 0..e3 {
                prod *= 3;
            }
            for _ in 0..e5 {
                prod *= 5;
            }
            for _ in 0..e7 {
                prod *= 7;
            }
            return if sum != 0 && prod % sum == 0 { 1 } else { 0 };
        }

        let key = (pos, tight, started, has_zero, sum, e2, e3, e5, e7);
        if let Some(&res) = self.dp.get(&key) {
            return res;
        }

        let mut res = 0;
        let limit = if tight { self.s[pos] as i32 - '0' as i32 } else { 9 };

        for d in 0..=limit {
            let ntight = tight && (d == limit);
            if !started {
                if d == 0 {
                    res += self.rec(pos + 1, ntight, false, false, 0, 0, 0, 0, 0);
                } else {
                    let mut new_e2 = 0;
                    let mut new_e3 = 0;
                    let mut new_e5 = 0;
                    let mut new_e7 = 0;
                    let mut temp = d;
                    while temp % 2 == 0 {
                        new_e2 += 1;
                        temp /= 2;
                    }
                    while temp % 3 == 0 {
                        new_e3 += 1;
                        temp /= 3;
                    }
                    while temp % 5 == 0 {
                        new_e5 += 1;
                        temp /= 5;
                    }
                    while temp % 7 == 0 {
                        new_e7 += 1;
                        temp /= 7;
                    }
                    res += self.rec(pos + 1, ntight, true, false, d, new_e2, new_e3, new_e5, new_e7);
                }
            } else {
                let new_sum = sum + d;
                if has_zero {
                    res += self.rec(pos + 1, ntight, true, true, new_sum, 0, 0, 0, 0);
                } else {
                    if d == 0 {
                        res += self.rec(pos + 1, ntight, true, true, new_sum, 0, 0, 0, 0);
                    } else {
                        let mut new_e2 = e2;
                        let mut new_e3 = e3;
                        let mut new_e5 = e5;
                        let mut new_e7 = e7;
                        let mut temp = d;
                        while temp % 2 == 0 {
                            new_e2 += 1;
                            temp /= 2;
                        }
                        while temp % 3 == 0 {
                            new_e3 += 1;
                            temp /= 3;
                        }
                        while temp % 5 == 0 {
                            new_e5 += 1;
                            temp /= 5;
                        }
                        while temp % 7 == 0 {
                            new_e7 += 1;
                            temp /= 7;
                        }
                        res += self.rec(
                            pos + 1,
                            ntight,
                            true,
                            false,
                            new_sum,
                            new_e2,
                            new_e3,
                            new_e5,
                            new_e7,
                        );
                    }
                }
            }
        }

        self.dp.insert(key, res);
        res
    }

    fn count_beautiful(&mut self, x: i32) -> i64 {
        if x < 1 {
            return 0;
        }
        self.dp.clear();
        self.s = x.to_string().chars().collect();
        self.rec(0, true, false, false, 0, 0, 0, 0, 0)
    }
}

impl Solution {
    pub fn beautiful_numbers(l: i32, r: i32) -> i32 {
        let mut solver = BeautifulNumbersSolver {
            s: vec![],
            dp: HashMap::new(),
        };
        (solver.count_beautiful(r) - solver.count_beautiful(l - 1)) as i32
    }
}
