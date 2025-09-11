class Solution {
public:
    int canBeTypedWords(string text, string brokenLetters) {
       unordered_set<char> brokenSet(brokenLetters.begin(), brokenLetters.end());
        int count = 0;
        int start = 0, n = text.size();
        for (int i = 0; i <= n; ++i) {
            if (i == n || text[i] == ' ') {
                bool canType = true;
                for (int j = start; j < i; ++j) {
                    if (brokenSet.count(text[j])) {
                        canType = false;
                        break;
                    }
                }
                if (canType) count++;
                start = i + 1;
            }
        }

        return count; 
    }
};