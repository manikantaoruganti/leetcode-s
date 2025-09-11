class Solution {
public:
    vector<vector<int>> flipAndInvertImage(vector<vector<int>>&m) {
        const int n = m.size();

    for (int i = 0; i < n; ++i)
      for (int j = 0; j < (n + 1) / 2; ++j) {
        const int temp = m[i][j];
        m[i][j] = m[i][n - j - 1] ^ 1;
        m[i][n - j - 1] = temp ^ 1;
      }

    return m;
    }
};