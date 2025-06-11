class Solution {
public:
    int mostFrequent(vector<int>& nums, int key) {
        
    unordered_map<int,int>m;
for(int i=0;i<nums.size()-1;i++)if(nums[i]==key)m[nums[i+1]]++;
int x=0,y=0;
for(auto[a,b]:m)if(b>y)y=b,x=a;
return x;
    }
};