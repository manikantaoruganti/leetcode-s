class Solution {
public:
    long long findTheArrayConcVal(vector<int>& nums) {
        long long res = 0;
for (int i = 0, j = (int)nums.size() - 1; i <= j; i++, j--)
    res += i == j ? nums[i] : stoll(to_string(nums[i]) + to_string(nums[j]));
return res;
        
    }
};