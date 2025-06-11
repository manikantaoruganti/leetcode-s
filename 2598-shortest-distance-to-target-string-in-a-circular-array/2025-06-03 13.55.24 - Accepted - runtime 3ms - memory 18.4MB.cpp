class Solution {
public:
    int closestTarget(vector<string>& words, string target, int startIndex) {
        int n = words.size();
int res = n + 1;
for (int i = 0; i < n; i++) {
    if (words[i] == target) {
        int dist = abs(i - startIndex);
        dist = min(dist, n - dist);
        if (dist < res) res = dist;
    }
}
return res == n + 1 ? -1 : res;
        
    }
};