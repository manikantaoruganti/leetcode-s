class Solution {
public:
    vector<int> findColumnWidth(vector<vector<int>>& grid) {
        int m = grid.size(), n = grid[0].size();
        vector<int> ans(n, 0);
        
        for (int col = 0; col < n; ++col) {
            for (int row = 0; row < m; ++row) {
                int len = to_string(grid[row][col]).size();
                ans[col] = max(ans[col], len);
            }
        }
        return ans;
    }
};
