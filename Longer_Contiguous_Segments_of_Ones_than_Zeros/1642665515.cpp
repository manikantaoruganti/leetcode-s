class Solution {
public:
    bool checkZeroOnes(string s) {
        int maxOnes = 0, maxZeros = 0, count = 1;
        for (int i = 1; i <= (int)s.size(); ++i) {
            if (i < (int)s.size() && s[i] == s[i-1]) {
                ++count;
            } else {
                if (s[i-1] == '1') maxOnes = max(maxOnes, count);
                else maxZeros = max(maxZeros, count);
                count = 1;
            }
        }
        return maxOnes > maxZeros;
    }
};
