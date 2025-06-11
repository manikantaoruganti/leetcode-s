class Solution {
public:
    int maximizeSum(vector<int>& nums, int k) {
        
    priority_queue<int> pq(nums.begin(), nums.end());
        int score = 0;

        for (int i = 0; i < k; i++) {
            int top = pq.top();
            pq.pop();
            score += top;
            pq.push(top + 1);  // insert incremented element back
        }

        return score;
    }
};