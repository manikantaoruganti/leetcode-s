class Solution {
public:
    int countHomogenous(string s) {
        const int MOD = 1'000'000'007;
        long long result = 0, count = 1;
        for (int i = 1; i <= s.size(); i++) {
            if (i < s.size() && s[i] == s[i - 1]) {
                count++;
            } else {
                
                result = (result + (count * (count + 1) / 2) % MOD) % MOD;
                count = 1;
            }
        }
        return (int)result;
    }
};
