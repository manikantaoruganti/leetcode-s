class Solution {
public:
    int distinctAverages(vector<int>& nums) {
        sort(nums.begin(), nums.end());
unordered_set<double> averages;
int i = 0, j = nums.size() - 1;
while (i < j) {
    averages.insert((nums[i++] + nums[j--]) / 2.0);
}
return averages.size();
        
    }
};