class Solution {
public:
    vector<bool> prefixesDivBy5(vector<int>& nums) {
        vector<bool> res;
        int val = 0;
        for (int bit : nums) {
            val = ((val << 1) + bit) % 5;
            res.push_back(val == 0);
        }
        return res;
    }
};
