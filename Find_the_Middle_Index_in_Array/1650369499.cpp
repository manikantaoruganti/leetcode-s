class Solution {
public:
    int findMiddleIndex(vector<int>& nums) {
        int total = accumulate(nums.begin(), nums.end(), 0), left = 0;
for (int i = 0; i < nums.size(); ++i) {
    if (left == total - left - nums[i]) return i;
    left += nums[i];
}
return -1;
        
    }
};