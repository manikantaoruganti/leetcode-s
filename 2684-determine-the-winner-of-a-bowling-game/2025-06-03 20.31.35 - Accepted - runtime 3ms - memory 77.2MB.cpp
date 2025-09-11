class Solution {
public:
    int isWinner(vector<int>& player1, vector<int>& player2) {
        int n = player1.size();
        int score1 = 0, score2 = 0;
        
        for (int i = 0; i < n; i++) {
            int val1 = player1[i];
            // Double if either of the previous two turns were 10
            if ((i - 1 >= 0 && player1[i-1] == 10) || (i - 2 >= 0 && player1[i-2] == 10))
                val1 *= 2;
            score1 += val1;
            
            int val2 = player2[i];
            if ((i - 1 >= 0 && player2[i-1] == 10) || (i - 2 >= 0 && player2[i-2] == 10))
                val2 *= 2;
            score2 += val2;
        }
        
        if (score1 > score2) return 1;
        else if (score2 > score1) return 2;
        else return 0;
    
    }
};