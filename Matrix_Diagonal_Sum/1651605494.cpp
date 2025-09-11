class Solution {
public:
    int diagonalSum(vector<vector<int>>& mat) {
        int n = mat.size(), sum = 0;
for (int i = 0; i < n; ++i) {
    sum += mat[i][i];               // primary diagonal
    sum += mat[i][n - 1 - i];       // secondary diagonal
}
if (n % 2 == 1) sum -= mat[n / 2][n / 2]; // center counted twice
return sum;
        
    }
};