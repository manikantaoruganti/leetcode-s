class Solution {
public:
    string firstPalindrome(vector<string>& words) {
        for (string& s : words) {
    int l = 0, r = s.size() - 1;
    while (l < r && s[l] == s[r]) {
        l++;
        r--;
    }
    if (l >= r) return s;
}
return "";
        
    }
};