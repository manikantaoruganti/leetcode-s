class Solution {
 public:
  vector<int> findSubstring(string s, vector<string>& words) {
    if (s.empty() || words.empty())
      return {};

    const int k = words.size();
    const int n = words[0].length();
    const int totalLen = k * n;

    if (s.length() < totalLen)
      return {};

    vector<int> ans;
    unordered_map<string, int> count;

    for (const string& word : words)
      ++count[word];

    for (int i = 0; i <= int(s.length()) - totalLen; ++i) {
      unordered_map<string, int> seen;
      int j = 0;
      while (j < k) {
        string word = s.substr(i + j * n, n);
        if (++seen[word] > count[word])
          break;
        ++j;
      }
      if (j == k)
        ans.push_back(i);
    }

    return ans;
  }
};
