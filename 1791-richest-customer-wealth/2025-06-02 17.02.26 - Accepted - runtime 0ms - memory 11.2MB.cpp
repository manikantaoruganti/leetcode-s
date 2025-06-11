class Solution {
public:
    int maximumWealth(vector<vector<int>>& accounts) {
        int maxWealth = 0;
for (auto& customer : accounts) {
    int wealth = accumulate(customer.begin(), customer.end(), 0);
    maxWealth = max(maxWealth, wealth);
}
return maxWealth;
        
    }
};