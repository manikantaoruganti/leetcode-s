class Solution {
public:
    int minimumCost(vector<int>& nums) {
        
    int n = nums.size(), res = INT_MAX;
    for (int i = 1; i < n - 1; i++)
        for (int j = i + 1; j < n; j++)
            res = min(res, nums[0] + nums[i] + nums[j]);
    return res;
    }
};