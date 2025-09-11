class Solution {
public:
    int countPrimes(int n) {
        if (n <= 2) return 0;

        vector<int> primes;
        primes.push_back(2);

        for (int i = 3; i < n; i += 2) { 
            bool isPrime = true;

        
            for (int p : primes) {
                if ((long long)p * p > i) break;
                if (i % p == 0) {
                    isPrime = false;
                    break;
                }
            }

            if (isPrime) primes.push_back(i);
        }

        return primes.size();
    }
};
