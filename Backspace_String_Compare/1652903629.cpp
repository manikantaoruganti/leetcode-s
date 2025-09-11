class Solution {
    string process(string str) {
    string res;
    for (char c : str) {
        if (c == '#') {
            if (!res.empty()) res.pop_back();
        } else {
            res.push_back(c);
        }
    }
    return res;
    }
public:
    bool backspaceCompare(string s, string t) {
       return process(s) == process(t); 
    }
};