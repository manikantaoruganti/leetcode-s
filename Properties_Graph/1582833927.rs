/*impl Solution {
    pub fn number_of_components(properties: Vec<Vec<i32>>, k: i32) -> i32 {
        
    }
}*/

 use std::collections::{HashSet, VecDeque};

impl Solution {
    pub fn number_of_components(properties: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = properties.len();
        let mut graph = vec![vec![]; n];

        // Build the adjacency list
        for i in 0..n {
            for j in (i + 1)..n {
                let set_i: HashSet<_> = properties[i].iter().collect();
                let set_j: HashSet<_> = properties[j].iter().collect();
                let common = set_i.intersection(&set_j).count() as i32;

                if common >= k {
                    graph[i].push(j);
                    graph[j].push(i);
                }
            }
        }

        // BFS/DFS to count connected components
        let mut visited = vec![false; n];
        let mut components = 0;

        for i in 0..n {
            if !visited[i] {
                components += 1;
                let mut queue = VecDeque::new();
                queue.push_back(i);
                visited[i] = true;

                while let Some(node) = queue.pop_front() {
                    for &neighbor in &graph[node] {
                        if !visited[neighbor] {
                            visited[neighbor] = true;
                            queue.push_back(neighbor);
                        }
                    }
                }
            }
        }

        components
    }
}
