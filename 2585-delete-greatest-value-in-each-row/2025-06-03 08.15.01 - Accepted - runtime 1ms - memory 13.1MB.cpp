class Solution {
public:
    int deleteGreatestValue(vector<vector<int>>& grid) {
        int ans=0;for(auto&row:grid)sort(row.begin(),row.end());for(int i=grid[0].size()-1;i>=0;i--){int m=0;for(auto&row:grid)m=max(m,row[i]);ans+=m;}return ans;
        
    }
};