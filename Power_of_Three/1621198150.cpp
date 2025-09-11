#include <cmath>
class Solution {
    public:
bool isPowerOfThree(int n) {
    if (n <= 0) return false;
    double log3n = log10(n) / log10(3);
    return floor(log3n) == log3n;
}
};