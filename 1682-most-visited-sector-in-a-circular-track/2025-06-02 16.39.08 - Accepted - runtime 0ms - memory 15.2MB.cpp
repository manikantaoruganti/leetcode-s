class Solution {
public:
    vector<int> mostVisited(int n, vector<int>& rounds) {
        vector<int> count(n + 1, 0);
int m = rounds.size();

count[rounds[0]]++;

for (int i = 1; i < m; ++i) {
    int start = rounds[i - 1], end = rounds[i];
    int curr = start;
    while (curr != end) {
        curr = curr % n + 1;
        count[curr]++;
    }
}

int maxVisits = *max_element(count.begin() + 1, count.end());
vector<int> result;
for (int i = 1; i <= n; ++i)
    if (count[i] == maxVisits)
        result.push_back(i);
return result;
        
    }
};