class Solution {
    static constexpr int MOD = 1'000'000'007;
    

    int dp[25][401][2];
    string num;
    int maxSum;
    int dfs(int pos, int sum, bool tight) {
        if (sum > maxSum) return 0;  
        if (pos == (int)num.size()) return 1;  
        
        if (dp[pos][sum][tight] != -1) return dp[pos][sum][tight];
        
        int limit = tight ? (num[pos] - '0') : 9;
        int res = 0;
        
        for (int dig = 0; dig <= limit; dig++) {
            res += dfs(pos + 1, sum + dig, tight && (dig == limit));
            res %= MOD;
        }
        
        return dp[pos][sum][tight] = res;
    }
    
    
    int countLessOrEqual(string s, int maxSumVal) {
        if (maxSumVal < 0) return 0;
        num = s;
        maxSum = maxSumVal;
        memset(dp, -1, sizeof(dp));
        return dfs(0, 0, true);
    }
    
    
    string minusOne(string s) {
        int n = s.size();
        int i = n - 1;
        while (i >= 0) {
            if (s[i] > '0') {
                s[i]--;
                break;
            } else {
                s[i] = '9';
                i--;
            }
        }
        if (s[0] == '0' && s.size() > 1) s.erase(0, 1);
        return s;
    }
    
public:
    int count(string num1, string num2, int min_sum, int max_sum) {
        int A = countLessOrEqual(num2, max_sum);
        int B = countLessOrEqual(num2, min_sum - 1);
        string num1_minus = minusOne(num1);
        int C = countLessOrEqual(num1_minus, max_sum);
        int D = countLessOrEqual(num1_minus, min_sum - 1);
        
        int res = ((A - B - C + D) % MOD + MOD) % MOD;
        return res;
    }
};
