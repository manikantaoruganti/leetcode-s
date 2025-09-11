class Solution {
public:
    int countGoodRectangles(vector<vector<int>>& rectangles) {
        int maxLen = 0, count = 0;
for (auto& r : rectangles) {
    int len = min(r[0], r[1]);
    if (len > maxLen) maxLen = len, count = 1;
    else if (len == maxLen) count++;
}
return count;
        
    }
};