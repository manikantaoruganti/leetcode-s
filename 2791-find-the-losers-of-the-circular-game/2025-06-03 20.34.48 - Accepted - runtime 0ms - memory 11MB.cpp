class Solution {
public:
    vector<int> circularGameLosers(int n, int k) {
        vector<bool> visited(n + 1, false);
        int current = 1, step = 1;
        visited[current] = true;
        while (true) {
            current = (current + step * k - 1) % n + 1;
            if (visited[current]) break;
            visited[current] = true;
            step++;
        }
        vector<int> losers;
        for (int i = 1; i <= n; i++) {
            if (!visited[i]) losers.push_back(i);
        }
        return losers;
    }
};