class Solution {
public:
    int largestAltitude(vector<int>& gain) {
        int alt = 0, maxAlt = 0;
for (int g : gain) {
    alt += g;
    if (alt > maxAlt) maxAlt = alt;
}
return maxAlt;
        
    }
};