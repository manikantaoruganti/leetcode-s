class Solution {
public:
    int semiOrderedPermutation(vector<int>& nums) {
        int n = nums.size();
        int pos1 = 0, posn = 0;
        for (int i = 0; i < n; i++) {
            if (nums[i] == 1) pos1 = i;
            if (nums[i] == n) posn = i;
        }
        int res = pos1 + (n - 1 - posn);
        if (pos1 > posn) res--;
        return res;
    }
};