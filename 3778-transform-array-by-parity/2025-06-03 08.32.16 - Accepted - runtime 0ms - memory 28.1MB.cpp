class Solution {
public:
    vector<int> transformArray(vector<int>& nums) {
    vector<int> res;
    for(int x:nums) res.push_back(x%2?1:0);
    sort(res.begin(),res.end());
    return res;
}

};