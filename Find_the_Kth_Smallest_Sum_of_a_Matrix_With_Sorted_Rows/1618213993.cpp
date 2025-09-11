/*class Solution {
public:
    int kthSmallest(vector<vector<int>>& mat, int k) {
        
    }
};*/
#include <vector>
#include <queue>
#include <set>
using namespace std;

class Solution {
public:
    int kthSmallest(vector<vector<int>>& mat, int k) {
        int m = mat.size(), n = mat[0].size();
        
        // Start with sum of picking the first element from each row
        int initial_sum = 0;
        vector<int> indices(m, 0); // indices[i] = 0 means picking mat[i][0]
        for (int i = 0; i < m; ++i) {
            initial_sum += mat[i][0];
        }
        
        // Min-heap: (sum, indices vector)
        using T = pair<int, vector<int>>;
        priority_queue<T, vector<T>, greater<T>> pq;
        set<vector<int>> visited;
        
        pq.push({initial_sum, indices});
        visited.insert(indices);
        
        // Perform k-1 times pop to get the k-th smallest
        while (--k) {
            auto [sum, idx] = pq.top();
            pq.pop();
            
            // Try to move to next element in each row
            for (int i = 0; i < m; ++i) {
                vector<int> next_idx = idx;
                if (next_idx[i] + 1 < n) {
                    ++next_idx[i];
                    if (visited.insert(next_idx).second) { // only if not visited
                        int next_sum = sum - mat[i][idx[i]] + mat[i][next_idx[i]];
                        pq.push({next_sum, next_idx});
                    }
                }
            }
        }
        
        return pq.top().first;
    }
};
