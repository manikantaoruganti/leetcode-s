class Solution {
public:
    int countDigitOne(int n) {
        long long count = 0;
        for (long long pos = 1; pos <= n; pos *= 10) {
            long long high = n / (pos * 10);
            long long curr = (n / pos) % 10;
            long long low = n % pos;
            
            if (curr == 0) {
                count += high * pos;
            } else if (curr == 1) {
                count += high * pos + (low + 1);
            } else {
                count += (high + 1) * pos;
            }
        }
        return count;
    }
};
