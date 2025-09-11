class Solution {
public:
    string generateTheString(int n) {
        
    string result(n, 'a');
        if (n % 2 == 0) {
            
            result[n - 1] = 'b';
        }
        return result;
    }
};