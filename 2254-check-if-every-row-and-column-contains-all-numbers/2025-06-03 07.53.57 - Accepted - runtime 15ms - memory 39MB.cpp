class Solution {
public:
    bool checkValid(vector<vector<int>>& matrix) {
      int n = matrix.size();
for (int i = 0; i < n; ++i) {
    vector<bool> row(n + 1, false), col(n + 1, false);
    for (int j = 0; j < n; ++j) {
        if (row[matrix[i][j]] || col[matrix[j][i]])
            return false;
        row[matrix[i][j]] = true;
        col[matrix[j][i]] = true;
    }
}
return true;
        
    }
};