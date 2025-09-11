/*impl Solution {
    pub fn min_costs(cost: Vec<i32>) -> Vec<i32> {
        
    }
}*/ 
impl Solution {
    pub fn min_costs(cost: Vec<i32>) -> Vec<i32> {
        let n = cost.len();
        let mut answer = vec![i32::MAX; n];
        
        let mut min_cost = i32::MAX;
        for i in 0..n {
            min_cost = min_cost.min(cost[i]);
            answer[i] = min_cost;
        }
        
        answer
    }
}