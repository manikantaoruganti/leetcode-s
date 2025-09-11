class Solution {
public:
    int minMaxGame(vector<int>& nums) {
      while (nums.size() > 1) {
    vector<int> temp(nums.size() / 2);
    for (int i = 0; i < temp.size(); ++i)
        temp[i] = i % 2 ? max(nums[2 * i], nums[2 * i + 1]) : min(nums[2 * i], nums[2 * i + 1]);
    nums = temp;
}
return nums[0];
        
    }
};