class Solution {
public:
    string freqAlphabets(string s) {
        string result;
        int i = 0;
        while (i < s.size()) {
            if (i + 2 < s.size() && s[i+2] == '#') {
                int num = (s[i] - '0') * 10 + (s[i+1] - '0');
                result += (char)('a' + num - 1);
                i += 3;
            } else {
                result += (char)('a' + (s[i] - '0') - 1);
                i++;
            }
        }
        return result;
    }
};