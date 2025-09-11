class Solution {
public:
    bool makesquare(vector<int>& matchsticks) {
        int n = matchsticks.size();
        int total = accumulate(matchsticks.begin(), matchsticks.end(), 0);
        if (total % 4 != 0) return false;

        int side = total / 4;
        vector<int> sides(4, 0);
        sort(matchsticks.rbegin(), matchsticks.rend()); 

        function<bool(int)> backtrack = [&](int idx) {
            if (idx == n) {
                return sides[0] == side && sides[1] == side && sides[2] == side && sides[3] == side;
            }
            for (int i = 0; i < 4; ++i) {
                if (sides[i] + matchsticks[idx] <= side) {
                    sides[i] += matchsticks[idx];
                    if (backtrack(idx + 1)) return true;
                    sides[i] -= matchsticks[idx];
        }
                if (sides[i] == 0) break;
            }
            return false;
        };

        return backtrack(0);
    }
};

