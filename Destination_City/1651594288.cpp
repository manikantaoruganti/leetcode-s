class Solution {
public:
    string destCity(vector<vector<string>>& paths) {
    unordered_set<string> from;
for(auto& p : paths) from.insert(p[0]);
for(auto& p : paths) if(!from.count(p[1])) return p[1];
return "";
        
    }
};