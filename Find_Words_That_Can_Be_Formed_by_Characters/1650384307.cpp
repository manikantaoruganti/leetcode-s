class Solution {
public:
    int countCharacters(vector<string>& words, string chars) {
        int res = 0;
vector<int> freq(26);
for (char c : chars) freq[c - 'a']++;
for (string& word : words) {
    vector<int> temp = freq;
    bool good = true;
    for (char c : word) {
        if (--temp[c - 'a'] < 0) {
            good = false;
            break;
        }
    }
    if (good) res += word.size();
}
return res;
        
    }
};