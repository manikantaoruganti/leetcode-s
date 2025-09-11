class Solution {
public:
    bool hasGroupsSizeX(vector<int>& deck) {
        unordered_map<int, int> freq;
for (int num : deck) freq[num]++;

int g = 0;
for (auto& [_, count] : freq) g = gcd(g, count);

return g >= 2;
        
    }
};