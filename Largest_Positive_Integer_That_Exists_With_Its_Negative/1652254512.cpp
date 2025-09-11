class Solution {
public:
    int findMaxK(vector<int>& nums) {
        unordered_set<int> s(nums.begin(), nums.end());
int maxK = -1;
for (int x : nums)
    if (s.count(-x))
        maxK = max(maxK, abs(x));
return maxK;
        
    }
};