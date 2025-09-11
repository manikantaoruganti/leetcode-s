class Solution {
public:
    bool isAlienSorted(vector<string>& words, string order) {
        int pos[26];
        for(int i=0; i<26; ++i) pos[order[i] - 'a'] = i;
        for(int i=1; i<(int)words.size(); ++i){
            string &a = words[i-1], &b = words[i];
            int j = 0;
            while(j < (int)a.size() && j < (int)b.size() && a[j] == b[j]) ++j;
            if(j == (int)b.size() && (int)a.size() > (int)b.size()) return false;
            if(j < (int)a.size() && j < (int)b.size() && pos[a[j] - 'a'] > pos[b[j] - 'a']) return false;
        }
        return true;
    }
};
