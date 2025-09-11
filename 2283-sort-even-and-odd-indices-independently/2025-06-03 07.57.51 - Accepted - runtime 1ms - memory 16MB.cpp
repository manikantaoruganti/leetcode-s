class Solution {
public:
    vector<int> sortEvenOdd(vector<int>& nums) {
      vector<int> even, odd;
for (int i = 0; i < nums.size(); i++) {
    if (i % 2 == 0) even.push_back(nums[i]);
    else odd.push_back(nums[i]);
}
sort(even.begin(), even.end());
sort(odd.begin(), odd.end(), greater<int>());
int evenIndex = 0, oddIndex = 0;
for (int i = 0; i < nums.size(); i++) {
    if (i % 2 == 0) nums[i] = even[evenIndex++];
    else nums[i] = odd[oddIndex++];
}
return nums;
        
    }
};