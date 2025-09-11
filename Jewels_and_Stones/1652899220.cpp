class Solution {
public:
    int numJewelsInStones(string jewels, string stones) {
        unordered_set<char> s(jewels.begin(), jewels.end());
return count_if(stones.begin(), stones.end(), [&](char c){ return s.count(c); });
        
    }
};