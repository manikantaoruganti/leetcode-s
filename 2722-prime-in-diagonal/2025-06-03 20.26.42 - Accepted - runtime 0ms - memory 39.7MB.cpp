class Solution {
public:
    bool isPrime(int n) {
        if (n < 2) return false;
        if (n == 2 || n == 3) return true;
        if (n % 2 == 0 || n % 3 == 0) return false;
        for (int i = 5; i * 1LL * i <= n; i += 6) {
            if (n % i == 0 || n % (i + 2) == 0)
                return false;
        }
        return true;
    }

    int diagonalPrime(vector<vector<int>>& nums) {
        int n = nums.size();
        int maxPrime = 0;

        for (int i = 0; i < n; ++i) {
            int mainDiag = nums[i][i];
            int antiDiag = nums[i][n - i - 1];

            if (isPrime(mainDiag)) maxPrime = max(maxPrime, mainDiag);
            if (isPrime(antiDiag)) maxPrime = max(maxPrime, antiDiag);
        }

        return maxPrime;
    }
};
