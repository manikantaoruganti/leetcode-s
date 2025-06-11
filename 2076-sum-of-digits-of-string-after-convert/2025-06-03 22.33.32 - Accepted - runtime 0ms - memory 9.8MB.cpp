class Solution {
public:
    int getLucky(string s, int k) {
        string numStr;
        for (char c : s) {
            int val = c - 'a' + 1;
            numStr += to_string(val);
        }
        
        for (int i = 0; i < k; i++) {
            int sum = 0;
            for (char ch : numStr) {
                sum += ch - '0';
            }
            numStr = to_string(sum);
        }
        
        return stoi(numStr);
    }
};