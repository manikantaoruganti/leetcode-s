
class Solution {
public:
    int maxProfit(vector<int>& prices) {
        const int n = prices.size();
        if (n < 2) return 0;

        int minPrice = prices[0];
        int maxProfit = 0;

        for (int i = 1; i < n; ++i) {
            int price = prices[i];

            // Inline comparisons to reduce branching
            if (price < minPrice) {
                minPrice = price;
            } else {
                int profit = price - minPrice;
                if (profit > maxProfit) {
                    maxProfit = profit;
                }
            }
        }

        return maxProfit;
    }
};
