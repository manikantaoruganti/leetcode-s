class Solution {
public:
    int countPrefixes(vector<string>& words, string s) {
       int res = 0;
for (int i = 0; i < words.size(); ++i)
    if (s.compare(0, words[i].size(), words[i]) == 0)
        ++res;
return res;
        
    }
};