class Solution {
public:
    int findLengthOfLCIS(vector<int>& nums) {
        int res = 1, cur = 1;
for (int i = 1; i < nums.size(); ++i)
    res = max(res, cur = nums[i] > nums[i - 1] ? cur + 1 : 1);
return res;
        
    }
};