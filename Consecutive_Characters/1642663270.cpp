class Solution {
public:
    int maxPower(string s) {
        int max_len = 1, cur_len = 1;
        for (int i = 1; i < s.size(); i++) {
            if (s[i] == s[i - 1]) {
                cur_len++;
            } else {
                max_len = max(max_len, cur_len);
                cur_len = 1;
            }
        }
        return max(max_len, cur_len);
    }
};
