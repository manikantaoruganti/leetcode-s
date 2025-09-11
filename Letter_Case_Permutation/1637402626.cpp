
class Solution {
public:
    vector<string> letterCasePermutation(string s) {
        vector<int> alpha_pos;
        for (int i = 0; i < s.size(); ++i)
            if (isalpha(s[i]))
                alpha_pos.push_back(i);

        int total = 1 << alpha_pos.size(); // 2^k permutations
        vector<string> res;

        for (int mask = 0; mask < total; ++mask) {
            string temp = s;
            for (int bit = 0; bit < alpha_pos.size(); ++bit) {
                if ((mask >> bit) & 1)
                    temp[alpha_pos[bit]] ^= (1 << 5); // flip case
            }
            res.push_back(temp);
        }
        return res;
    }
};
