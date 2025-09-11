class Solution {
public:
    int dominantIndex(vector<int>& nums) {
       int maxVal = -1, idx = -1;
for (int i = 0; i < nums.size(); ++i)
    if (nums[i] > maxVal) maxVal = nums[i], idx = i;
for (int i = 0; i < nums.size(); ++i)
    if (i != idx && maxVal < 2 * nums[i]) return -1;
return idx;
        
    }
};