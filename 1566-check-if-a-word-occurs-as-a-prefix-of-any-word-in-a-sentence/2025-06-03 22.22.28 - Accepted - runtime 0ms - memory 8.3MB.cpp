class Solution {
public:
    int isPrefixOfWord(string sentence, string searchWord) {
        int index = 1;
        int n = sentence.size();
        int m = searchWord.size();
        int start = 0;
        
        for (int i = 0; i <= n; ++i) {
            if (i == n || sentence[i] == ' ') {
                // word is from start to i-1
                if (i - start >= m && sentence.compare(start, m, searchWord) == 0) {
                    return index;
                }
                index++;
                start = i + 1;
            }
        }
        
        return -1;
    }
};