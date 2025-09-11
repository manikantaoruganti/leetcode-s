class Solution {
public:
    bool divideArray(vector<int>& nums) {
      unordered_map<int,int>m;
for(int x:nums)m[x]++;
for(auto&[k,v]:m)if(v%2)return false;
return true;
        
    }
};