class Solution {
public:
    int maxLengthBetweenEqualCharacters(string s) {
        int maxLen = -1;
        vector<int> firstPos(26, -1);

        for (int i = 0; i < (int)s.size(); i++) {
            int idx = s[i] - 'a';
            if (firstPos[idx] == -1) {
                firstPos[idx] = i;
            } else {
                int len = i - firstPos[idx] - 1;
                if (len > maxLen) maxLen = len;
            }
        }
        return maxLen;
    }
};