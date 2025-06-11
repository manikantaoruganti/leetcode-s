class Solution {
public:
    int mostWordsFound(vector<string>& sentences) {
        int maxWords = 0;
for (string& s : sentences) {
    int words = count(s.begin(), s.end(), ' ') + 1;
    maxWords = max(maxWords, words);
}
return maxWords;
        
    }
};