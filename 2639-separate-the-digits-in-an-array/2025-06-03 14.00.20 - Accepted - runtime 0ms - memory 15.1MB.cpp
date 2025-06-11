class Solution {
public:
    vector<int> separateDigits(vector<int>& nums) {
     vector<int> res;
for(auto x : nums){
    string s = to_string(x);
    for(char c : s) res.push_back(c - '0');
}
return res;
        
    }
};