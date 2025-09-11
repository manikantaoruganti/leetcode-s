class Solution {
public:
    vector<int> minSubsequence(vector<int>& nums) {
       sort(nums.begin(),nums.end(),greater<int>());
int total=accumulate(nums.begin(),nums.end(),0),sum=0;
vector<int>res;
for(int n:nums){
sum+=n;
res.push_back(n);
if(sum>total-sum)break;
}
return res;
        
    }
};