class Solution {
public:
    int findGCD(vector<int>& nums) {
        int mn = nums[0], mx = nums[0];
for (int num : nums) {
    if (num < mn) mn = num;
    if (num > mx) mx = num;
}

// Euclidean algorithm to find GCD of mn and mx
while (mx % mn != 0) {
    int temp = mx % mn;
    mx = mn;
    mn = temp;
}
return mn;
        
    }
};