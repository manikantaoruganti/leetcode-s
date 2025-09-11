class Solution {
public:
    vector<vector<int>> mergeSimilarItems(vector<vector<int>>& items1, vector<vector<int>>& items2) {
       map<int, int> weightMap;

for (auto &item : items1)
    weightMap[item[0]] += item[1];

for (auto &item : items2)
    weightMap[item[0]] += item[1];

vector<vector<int>> result;
for (auto &[value, weight] : weightMap)
    result.push_back({value, weight});

return result;
        
    }
};