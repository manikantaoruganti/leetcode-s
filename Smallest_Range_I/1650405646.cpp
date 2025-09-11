class Solution {
public:
    int smallestRangeI(vector<int>& nums, int k) {
       int minVal = *min_element(nums.begin(), nums.end());
        int maxVal = *max_element(nums.begin(), nums.end());
        int diff = maxVal - minVal;
        return max(0, diff - 2 * k); 
    }
};