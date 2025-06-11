class Solution {
public:
    vector<int> distributeCandies(int candies, int num_people) {
        
    vector<int> ans(num_people, 0);
        int give = 1, i = 0;
        while (candies > 0) {
            ans[i] += min(give, candies);
            candies -= give;
            give++;
            i = (i + 1) % num_people;
        }
        return ans;
    }
};