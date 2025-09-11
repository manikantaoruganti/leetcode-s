class Solution {
public:
    vector<string> uncommonFromSentences(string s1, string s2) {
        unordered_map<string, int> freq;
    istringstream iss(s1 + " " + s2);
    string word;
    while (iss >> word) freq[word]++;
    
    vector<string> res;
    for (auto& [w, c] : freq)
        if (c == 1) res.push_back(w);
    return res;
    }
};