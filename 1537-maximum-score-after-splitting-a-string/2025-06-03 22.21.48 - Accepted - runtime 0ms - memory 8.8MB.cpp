class Solution {
public:
    int maxScore(string s) {
        int n = s.size();
        int max_score = 0;
        int total_ones = 0;
        for (char c : s) if (c == '1') total_ones++;
        int zeros_in_left = 0, ones_in_right = total_ones;
        for (int i = 0; i < n - 1; ++i) {
            if (s[i] == '0') zeros_in_left++;
            else ones_in_right--;
            max_score = max(max_score, zeros_in_left + ones_in_right);
        }
        return max_score;
    }
};