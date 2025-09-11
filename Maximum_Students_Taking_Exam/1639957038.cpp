class Solution {
public:
    int maxStudents(vector<vector<char>>& seats) {
        int m = seats.size();
        int n = seats[0].size();
        vector<int> validRows(m, 0);

        // Preprocess each row into bitmask
        for (int i = 0; i < m; ++i) {
            int mask = 0;
            for (int j = 0; j < n; ++j) {
                if (seats[i][j] == '.') {
                    mask |= (1 << j);
                }
            }
            validRows[i] = mask;
        }

        unordered_map<int, int> dp;
        dp[0] = 0;

        for (int i = 0; i < m; ++i) {
            unordered_map<int, int> new_dp;
            for (int currMask = 0; currMask < (1 << n); ++currMask) {
                if ((currMask & validRows[i]) != currMask) continue; // using invalid seats
                if (currMask & (currMask >> 1)) continue; // adjacent students

                for (auto& [prevMask, val] : dp) {
                    if ((currMask & (prevMask >> 1)) || (currMask & (prevMask << 1))) continue; // cheating diagonally
                    int count = __builtin_popcount(currMask);
                    new_dp[currMask] = max(new_dp[currMask], val + count);
                }
            }
            dp = move(new_dp);
        }

        int result = 0;
        for (auto& [mask, val] : dp) {
            result = max(result, val);
        }
        return result;
    }
};
