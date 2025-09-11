   use std::collections::VecDeque;

struct Line {
    slope: i64,
    intercept: i64,
    x: f64,
}

impl Line {
    fn intersection(l1: &Line, l2: &Line) -> f64 {
        (l2.intercept - l1.intercept) as f64 / (l1.slope - l2.slope) as f64
    }
}



impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, cost: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut prefix_sum: Vec<i64> = vec![0; n + 1];
        let mut cost_sum: Vec<i64> = vec![0; n + 1];
        
        for i in 1..=n {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i - 1] as i64;
            cost_sum[i] = cost_sum[i - 1] + cost[i - 1] as i64;
        }
        
        let inf = i64::MAX / 2;
        let mut dp: Vec<i64> = vec![inf; n + 1];
        let mut prev_dp: Vec<i64> = vec![inf; n + 1];
        prev_dp[0] = 0;
        
        let mut result = inf;
        
        for j in 1..=n {
            let mut hull: VecDeque<Line> = VecDeque::new();
            let mut ptr = 0;
            
            {
                let mut add_line = |slope: i64, intercept: i64| {
                    let new_line = Line { slope, intercept, x: 0.0 };
                    while let Some(last) = hull.back() {
                        if last.slope == new_line.slope {
                            if last.intercept <= new_line.intercept {
                                return;
                            } else {
                                hull.pop_back();
                            }
                        } else {
                            let new_x = Line::intersection(last, &new_line);
                            if !hull.is_empty() && new_x <= last.x {
                                hull.pop_back();
                            } else {
                                let mut new_line = new_line;
                                new_line.x = new_x;
                                hull.push_back(new_line);
                                break;
                            }
                        }
                    }
                    if hull.is_empty() {
                        hull.push_back(Line { slope, intercept, x: f64::NEG_INFINITY });
                    }
                };
                
                for l in j - 1..n {
                    if prev_dp[l] < inf {
                        add_line(-cost_sum[l], prev_dp[l]);
                    }
                }
            }
            
            let mut query = |x: i64, hull: &VecDeque<Line>, ptr: &mut usize| -> i64 {
                while *ptr + 1 < hull.len() && hull[*ptr + 1].x <= x as f64 {
                    *ptr += 1;
                }
                let best = &hull[*ptr];
                best.slope * x + best.intercept
            };
            
            for i in j..=n {
                let x = prefix_sum[i] + (k as i64) * (j as i64);
                dp[i] = x * cost_sum[i] + query(x, &hull, &mut ptr);
            }
            
            prev_dp = dp.clone();
            result = result.min(dp[n]);
        }
        
        result
    }
}