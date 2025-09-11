class Solution {
public:
    int numIslands(vector<vector<char>>& grid) {
        if (grid.empty()) return 0;

        int islands = 0;
        int m = grid.size();
        int n = grid[0].size();
        vector<pair<int, int>> stack;

        for (int i = 0; i < m; ++i) {
            for (int j = 0; j < n; ++j) {
                if (grid[i][j] == '1') {
                    ++islands;
                    stack.emplace_back(i, j);
                    while (!stack.empty()) {
                        auto [x, y] = stack.back();
                        stack.pop_back();
                        if (x < 0 || y < 0 || x >= m || y >= n || grid[x][y] != '1') continue;

                        grid[x][y] = '0'; // Mark as visited

                        stack.emplace_back(x + 1, y);
                        stack.emplace_back(x - 1, y);
                        stack.emplace_back(x, y + 1);
                        stack.emplace_back(x, y - 1);
                    }
                }
            }
        }

        return islands;
    }
};
