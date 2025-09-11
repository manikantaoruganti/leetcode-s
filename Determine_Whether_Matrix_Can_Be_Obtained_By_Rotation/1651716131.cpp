class Solution {bool equal(vector<vector<int>>& a, vector<vector<int>>& b) {
    return a == b;
}
                void rotate(vector<vector<int>>& mat) {
    int n = mat.size();
    for (int i = 0; i < n / 2; ++i)
        for (int j = i; j < n - i - 1; ++j) {
            int tmp = mat[i][j];
            mat[i][j] = mat[n - 1 - j][i];
            mat[n - 1 - j][i] = mat[n - 1 - i][n - 1 - j];
            mat[n - 1 - i][n - 1 - j] = mat[j][n - 1 - i];
            mat[j][n - 1 - i] = tmp;
        }
                }
public:
    bool findRotation(vector<vector<int>>& mat, vector<vector<int>>& target) {
       



for (int i = 0; i < 4; ++i) {
    if (equal(mat, target)) return true;
    rotate(mat);
}
return false;
        
    }
};