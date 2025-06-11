class Solution {
public:
    int maxProduct(vector<int>& nums) {
       nth_element(nums.begin(), nums.end() - 2, nums.end());
int a = nums[nums.size() - 1], b = nums[nums.size() - 2];
return (a - 1) * (b - 1);
 
    }
};