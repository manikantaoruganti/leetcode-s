class Solution {
public:
    bool hasAlternatingBits(int n) {
        int previous = n % 2;
n /= 2;
while (n > 0) {
    int current = n % 2;
    if (current == previous) return false;
    previous = current;
    n /= 2;
}
return true;
        
    }
};