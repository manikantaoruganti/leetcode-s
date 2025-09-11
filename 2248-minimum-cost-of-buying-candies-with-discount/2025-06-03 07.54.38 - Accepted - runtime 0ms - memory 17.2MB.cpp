class Solution {
public:
    int minimumCost(vector<int>& cost) {
     sort(cost.rbegin(), cost.rend());
int total = 0;
for (int i = 0; i < cost.size(); ++i) {
    if ((i + 1) % 3 != 0)  // skip every 3rd candy (free one)
        total += cost[i];
}
return total;
        
    }
};