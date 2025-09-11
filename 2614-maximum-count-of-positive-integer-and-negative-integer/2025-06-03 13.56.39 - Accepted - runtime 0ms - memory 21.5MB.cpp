class Solution {
public:
    int maximumCount(vector<int>& nums) {
        int n=nums.size();
int left=0,right=n-1;
while(left<=right){
    int mid=left+(right-left)/2;
    if(nums[mid]<0)left=mid+1;
    else right=mid-1;
}
int neg=left;
left=0;right=n-1;
while(left<=right){
    int mid=left+(right-left)/2;
    if(nums[mid]<=0)left=mid+1;
    else right=mid-1;
}
int pos=n-left;
return neg>pos?neg:pos;
        
    }
};