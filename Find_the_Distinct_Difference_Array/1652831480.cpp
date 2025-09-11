class Solution {
public:
    vector<int> distinctDifferenceArray(vector<int>& nums) {
        int n=nums.size();vector<int>p(n),s(n+1);
unordered_set<int>se;
for(int i=0;i<n;i++){se.insert(nums[i]);p[i]=se.size();}
se.clear();
for(int i=n-1;i>=0;i--){se.insert(nums[i]);s[i]=se.size();}
vector<int>r(n);
for(int i=0;i<n;i++)r[i]=p[i]-s[i+1];
return r;
    }
};