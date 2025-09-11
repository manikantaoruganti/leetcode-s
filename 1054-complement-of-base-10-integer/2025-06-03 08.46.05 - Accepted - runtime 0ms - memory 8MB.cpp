class Solution {
public:
    int bitwiseComplement(int n) {
        if (n == 0) return 1;  // special case: binary "0" should become "1"
    int mask = 0, temp = n;
    while (temp) {
        mask = (mask << 1) | 1;
        temp >>= 1;
    }
    return n ^ mask;
    
    }
};