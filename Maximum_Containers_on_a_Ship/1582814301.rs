/*impl Solution {
    pub fn max_containers(n: i32, w: i32, max_weight: i32) -> i32 {
        
    }
}*/
  impl Solution {
    pub fn max_containers(n: i32, w: i32, max_weight: i32) -> i32 {
        let total_cells = n * n;
        let max_possible = max_weight / w;
        total_cells.min(max_possible) // The minimum of total cells and weight-limited containers
    }
}
