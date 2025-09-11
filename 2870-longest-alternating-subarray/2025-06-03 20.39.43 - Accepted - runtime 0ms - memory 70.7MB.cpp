class Solution {
public:
    int alternatingSubarray(vector<int>& nums) {
       int n = nums.size(), maxLen = -1;
        for (int i = 0; i < n - 1; ++i) {
            if (nums[i + 1] - nums[i] == 1) {
                int length = 2;
                bool expectIncrease = false; // next difference should be -1
                for (int j = i + 2; j < n; ++j) {
                    int diff = nums[j] - nums[j - 1];
                    if ((expectIncrease && diff == 1) || (!expectIncrease && diff == -1)) {
                        length++;
                        expectIncrease = !expectIncrease;
                    } else {
                        break;
                    }
                }
                maxLen = max(maxLen, length);
            }
        }
        return maxLen; 
    }
};