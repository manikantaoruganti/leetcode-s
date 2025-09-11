class Solution {
public:
    vector<int> answerQueries(vector<int>& nums, vector<int>& queries) {
       sort(nums.begin(), nums.end());
vector<int> prefix(nums.size());
prefix[0] = nums[0];
for (int i = 1; i < nums.size(); ++i)
    prefix[i] = prefix[i - 1] + nums[i];

vector<int> result;
for (int q : queries) {
    int left = 0, right = nums.size(), ans = 0;
    while (left < right) {
        int mid = (left + right) / 2;
        if (prefix[mid] <= q) {
            ans = mid + 1;
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    result.push_back(ans);
}
return result;
        
    }
};