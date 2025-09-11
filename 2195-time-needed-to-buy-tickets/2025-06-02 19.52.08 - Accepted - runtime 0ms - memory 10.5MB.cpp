class Solution {
public:
    int timeRequiredToBuy(vector<int>& tickets, int k) {
 int time = 0, target = tickets[k];
for (int i = 0; i < tickets.size(); ++i)
    time += min(tickets[i], i <= k ? target : target - 1);
return time;
        
    }
};