class Solution {
public:
    int minSideJumps(vector<int>& obstacles) {
        constexpr int nf = 1'000'000;
    vector<int> dp{nf, 1, 0, 1};
    for (const int obstacle : obstacles) {
      if (obstacle > 0)
        dp[obstacle] = nf;
      for (int i = 1; i <= 3; ++i)  // no
        if (i != obstacle)
for (int j = 1; j <= 3; ++j)  //before
         dp[i] = min({dp[i], dp[j] + (i == j ? 0 : 1)});

    }
 return ranges::min(dp);
    }
};