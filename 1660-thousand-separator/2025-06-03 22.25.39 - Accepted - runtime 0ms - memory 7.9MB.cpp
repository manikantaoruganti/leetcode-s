class Solution {
public:
    string thousandSeparator(int n) {
        string s = to_string(n);
        string res;
        int count = 0;
        
        for (int i = s.size() - 1; i >= 0; --i) {
            res.push_back(s[i]);
            count++;
            if (count == 3 && i != 0) {
                res.push_back('.');
                count = 0;
            }
        }
        
        reverse(res.begin(), res.end());
        return res;
    }
};