class Solution {
public:
    int maximumNumberOfStringPairs(vector<string>& words) {
        int count = 0;
        for (int i = 0; i < (int)words.size(); i++) {
            for (int j = i + 1; j < (int)words.size(); j++) {
                string rev = words[j];
                swap(rev[0], rev[1]);
                if (words[i] == rev) count++;
            }
        }
        return count;
    }
};