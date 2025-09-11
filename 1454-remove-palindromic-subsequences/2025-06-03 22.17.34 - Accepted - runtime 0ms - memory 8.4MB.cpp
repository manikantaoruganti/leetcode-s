class Solution {
public:
    int removePalindromeSub(string s) {
       int n = s.size();
        int left = 0, right = n - 1;
        while (left < right) {
            if (s[left] != s[right]) return 2;
            left++;
            right--;
        }
        return 1; 
    }
};