class Solution {
public:
    bool isCovered(vector<vector<int>>& ranges, int left, int right) {
       bool covered[51] = {};
for (auto& r : ranges)
    for (int i = r[0]; i <= r[1]; ++i)
        covered[i] = true;

for (int i = left; i <= right; ++i)
    if (!covered[i]) return false;

return true;
        
    }
};