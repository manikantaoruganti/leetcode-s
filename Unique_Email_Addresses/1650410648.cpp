class Solution {
public:
    int numUniqueEmails(vector<string>& emails) {
        unordered_set<string>n;

    for (const string& email : emails) {
      string local;
      for (const char c : email) {
        if (c == '+' || c == '@')
          break;
        if (c == '.')
          continue;
        local += c;
      }
      string atDomain = email.substr(email.find('@'));
      n.insert(local + atDomain);
    }

    return n.size();
    }
    
};