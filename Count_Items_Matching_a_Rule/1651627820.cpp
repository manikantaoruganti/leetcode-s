class Solution {
public:
    int countMatches(vector<vector<string>>& items, string ruleKey, string ruleValue) {
     int index = ruleKey == "type" ? 0 : ruleKey == "color" ? 1 : 2, count = 0;
for (auto& item : items)
    if (item[index] == ruleValue)
        count++;
return count;
        
    }
};