class Solution {
public:
    int countHillValley(vector<int>& nums) {
        int c=0;
for(int i=1;i<nums.size()-1;i++){
    if(nums[i]==nums[i-1])continue;
    int l=i-1,r=i+1;
    while(l>=0&&nums[l]==nums[i])l--;
    while(r<nums.size()&&nums[r]==nums[i])r++;
    if(l>=0&&r<nums.size()){
        if(nums[i]>nums[l]&&nums[i]>nums[r])c++;
        else if(nums[i]<nums[l]&&nums[i]<nums[r])c++;
    }
}
return c;
        
    }
};