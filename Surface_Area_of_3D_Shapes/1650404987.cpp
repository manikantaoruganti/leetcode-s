class Solution {
public:
    int surfaceArea(vector<vector<int>>& grid) {
        int area = 0;
int n = grid.size();

for (int i = 0; i < n; ++i) {
    for (int j = 0; j < n; ++j) {
        if (grid[i][j] > 0) {
            area += 6 * grid[i][j] - 2 * (grid[i][j] - 1); // surface area of vertical stack
            if (i > 0) area -= 2 * min(grid[i][j], grid[i - 1][j]); // overlap with top
            if (j > 0) area -= 2 * min(grid[i][j], grid[i][j - 1]); // overlap with left
        }
    }
}
return area;
        
    }
};