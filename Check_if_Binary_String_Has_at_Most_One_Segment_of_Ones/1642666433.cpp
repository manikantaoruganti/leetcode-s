class Solution {
public:
    bool checkOnesSegment(string s) {
        int segments = 0;
        for (int i = 0; i < (int)s.size(); ++i) {
            if (s[i] == '1' && (i == 0 || s[i-1] == '0')) {
                ++segments;
                if (segments > 1) return false;
            }
        }
        return true;
    }
};
