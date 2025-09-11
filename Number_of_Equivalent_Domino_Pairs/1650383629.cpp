class Solution {
public:
    int numEquivDominoPairs(vector<vector<int>>& dominoes) {
        unordered_map<int, int> count;
int res = 0;
for (auto& d : dominoes) {
    int key = min(d[0], d[1]) * 10 + max(d[0], d[1]);
    res += count[key]++;
}
return res;
        
    }
};