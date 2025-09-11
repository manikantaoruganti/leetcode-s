class Solution {
public:
    int countSpecialSubsequences(vector<int>& nums) {
        static const int MOD = 1'000'000'007;
        vector<long long> subseqCount(3, 0);

        for (int val : nums) {
            if (val == 0) {
                subseqCount[0] = (2 * subseqCount[0] + 1) % MOD;
            } else if (val == 1) {
                subseqCount[1] = (2 * subseqCount[1] + subseqCount[0]) % MOD;
            } else if (val == 2) {
                subseqCount[2] = (2 * subseqCount[2] + subseqCount[1]) % MOD;
            }
        }

        return static_cast<int>(subseqCount[2]);
    }
};
