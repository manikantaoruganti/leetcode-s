class Solution {
public:
    vector<int> lastVisitedIntegers(vector<int>& nums) {
        vector<int> seen;
        vector<int> ans;
        int k = 0; // counts consecutive -1s

        for (int num : nums) {
            if (num == -1) {
                k++;
                if (k <= seen.size()) {
                    ans.push_back(seen[k - 1]); // 0-based indexing
                } else {
                    ans.push_back(-1);
                }
            } else {
                seen.insert(seen.begin(), num); // prepend to seen
                k = 0; // reset counter for -1s
            }
        }
        return ans;
    }
};
