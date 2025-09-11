class Solution {
public:
    int numTrees(int n) {
   
        long long result = 1;

      
        for (int i = 0; i < n; ++i) {
            result *= (2 * n - i);
            result /= (i + 1);
        }


        result /= (n + 1);

        return (int) result;
    }
};
