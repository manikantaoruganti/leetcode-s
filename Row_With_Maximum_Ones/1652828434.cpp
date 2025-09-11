class Solution {
public:
    vector<int> rowAndMaximumOnes(vector<vector<int>>& mat) {
        int maxCount = -1, rowIndex = -1;
        for (int i = 0; i < mat.size(); ++i) {
            int count = 0;
            for (int val : mat[i]) {
                if (val == 1) ++count;
            }
            if (count > maxCount) {
                maxCount = count;
                rowIndex = i;
            }
        }
        return {rowIndex, maxCount};
        
    }
};