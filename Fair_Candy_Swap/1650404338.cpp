class Solution {
public:
    vector<int> fairCandySwap(vector<int>& aliceSizes, vector<int>& bobSizes) {
       int sumA = accumulate(aliceSizes.begin(), aliceSizes.end(), 0);
int sumB = accumulate(bobSizes.begin(), bobSizes.end(), 0);
int delta = (sumB - sumA) / 2;

unordered_set<int> setB(bobSizes.begin(), bobSizes.end());

for (int a : aliceSizes) {
    int b = a + delta;
    if (setB.count(b)) {
        return {a, b};
    }
}
return {};
        
    }
};