class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int n = prices.size();
        if (n < 2) return 0;

        int minPrice = prices[0];
        int maxProfit = 0;

        for (int i = 1; i < n; ++i) {
            minPrice = (prices[i] < minPrice) ? prices[i] : minPrice;
            int profit = prices[i] - minPrice;
            maxProfit = (profit > maxProfit) ? profit : maxProfit;
        }

        return maxProfit;
    }
};
