class Solution {
public:
    int projectionArea(vector<vector<int>>& grid) {
        int n = grid.size(), area = 0;
for (int i = 0; i < n; ++i) {
    int rowMax = 0, colMax = 0;
    for (int j = 0; j < n; ++j) {
        if (grid[i][j] > 0) area++;               // xy-plane (top view)
        rowMax = max(rowMax, grid[i][j]);         // zx-plane (side view)
        colMax = max(colMax, grid[j][i]);         // yz-plane (front view)
    }
    area += rowMax + colMax;
}
return area;
        
    }
};