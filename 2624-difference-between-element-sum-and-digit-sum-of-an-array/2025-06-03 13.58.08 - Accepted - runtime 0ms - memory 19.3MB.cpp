class Solution {
public:
    int differenceOfSum(vector<int>& nums) {
       int elementSum=0,digitSum=0;
for(int x:nums){
    elementSum+=x;
    int t=x;
    while(t){
        digitSum+=t%10;
        t/=10;
    }
}
return elementSum>digitSum?elementSum-digitSum:digitSum-elementSum;
        
    }
};