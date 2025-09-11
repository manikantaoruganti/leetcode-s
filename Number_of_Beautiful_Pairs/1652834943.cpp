class Solution {
public:
    int gcd(int a, int b) {
        while (b != 0) {
            int t = b;
            b = a % b;
            a = t;
        }
        return a;
    }
    int countBeautifulPairs(vector<int>& nums) {
        int res = 0;
        int n = nums.size();
        
        auto firstDigit = [](int x) {
            while (x >= 10) x /= 10;
            return x;
        };

        auto lastDigit = [](int x) {
            return x % 10;
        };

        for (int i = 0; i < n; i++) {
            int f = firstDigit(nums[i]);
            for (int j = i + 1; j < n; j++) {
                int l = lastDigit(nums[j]);
                if (gcd(f, l) == 1) res++;
            }
        }
        return res;
    }
};