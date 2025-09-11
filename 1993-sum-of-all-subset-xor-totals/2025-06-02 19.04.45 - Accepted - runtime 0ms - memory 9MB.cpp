class Solution {
    int dfs(int i, int xorSum, vector<int>& nums) {
    if (i == nums.size()) return xorSum;
    return dfs(i + 1, xorSum, nums) + dfs(i + 1, xorSum ^ nums[i], nums);
    }
    
public:
    int subsetXORSum(vector<int>& nums) {
        
return dfs(0, 0, nums);
        
    }
};