class Solution {
public:
    int countConsistentStrings(string allowed, vector<string>& words) {
        int count = 0;
unordered_set<char> allowedSet(allowed.begin(), allowed.end());
for (const string& word : words) {
    bool isValid = true;
    for (char c : word) {
        if (allowedSet.find(c) == allowedSet.end()) {
            isValid = false;
            break;
        }
    }
    if (isValid) count++;
}
return count;

    }
};