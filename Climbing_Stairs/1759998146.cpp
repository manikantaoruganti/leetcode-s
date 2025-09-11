class Solution {
public:
    int climbStairs(int n) {
       if(n==0) return 0;
        if(n==1) return 1;
        vector<int>dp(n+1);
        dp[0]=0;
        dp[1]=1;
        dp[2]=2;
        for(int j=3;j<=n;j++){
            dp[j]=dp[j-1]+dp[j-2];
        } return dp[n];
    }
};