class Solution {
public:
    int findMaxConsecutiveOnes(vector<int>& nums) {
        int maxCount = 0, count = 0;
        for (int n : nums)
            maxCount = n ? max(++count, maxCount) : (count = 0, maxCount);
        return maxCount;
    }
};
        
