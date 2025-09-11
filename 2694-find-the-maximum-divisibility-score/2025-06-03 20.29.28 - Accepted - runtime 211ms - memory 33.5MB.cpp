class Solution {
public:
    int maxDivScore(vector<int>& nums, vector<int>& divisors) {
     int maxScore = -1;
        int answer = INT_MAX;
        
        for (int d : divisors) {
            int score = 0;
            for (int num : nums) {
                if (num % d == 0) {
                    ++score;
                }
            }
            if (score > maxScore || (score == maxScore && d < answer)) {
                maxScore = score;
                answer = d;
            }
        }
        
        return answer;   
    }
};