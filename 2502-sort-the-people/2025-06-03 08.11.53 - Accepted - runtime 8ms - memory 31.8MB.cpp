class Solution {
public:
    vector<string> sortPeople(vector<string>& names, vector<int>& heights) {
     vector<pair<int, string>> people;
for (int i = 0; i < names.size(); ++i)
    people.push_back({heights[i], names[i]});

sort(people.rbegin(), people.rend()); // Sort by height descending

vector<string> result;
for (auto& [_, name] : people)
    result.push_back(name);

return result;
        
    }
};