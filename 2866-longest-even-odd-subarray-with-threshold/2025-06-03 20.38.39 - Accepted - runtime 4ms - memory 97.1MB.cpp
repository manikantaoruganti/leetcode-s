class Solution {
public:
    int longestAlternatingSubarray(vector<int>& nums, int threshold) {
       int n = nums.size();
        int maxLen = 0;

        for (int start = 0; start < n; ++start) {
            if (nums[start] % 2 != 0 || nums[start] > threshold) continue;

            int length = 1;
            for (int i = start + 1; i < n; ++i) {
                if (nums[i] > threshold) break;
                if ((nums[i] % 2) == (nums[i - 1] % 2)) break;
                length++;
            }
            maxLen = max(maxLen, length);
        }
        return maxLen; 
    }
};