class Solution {
public:
    int findClosestNumber(vector<int>& nums) {
        int x=nums[0];
for(int n:nums)if(abs(n)<abs(x)||(abs(n)==abs(x)&&n>x))x=n;
return x;
        
    }
};