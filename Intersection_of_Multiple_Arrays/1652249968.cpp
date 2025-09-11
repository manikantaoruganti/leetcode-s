class Solution {
    vector<int> f(vector<vector<int>>& a) {
        int c[1001] = {}, n = a.size();
        for (auto& v : a)
            for (int x : v)
                ++c[x];
        vector<int> r;
        for (int i = 1; i <= 1000; i++)
            if (c[i] == n)
                r.push_back(i);
        return r;
    }
    
public:
    vector<int> intersection(vector<vector<int>>& nums) {
        return f(nums);
    }
};
