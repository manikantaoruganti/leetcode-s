class Solution {
public:
    int maxDistance(vector<int>& colors) {
       int n = colors.size();
int a = 0, b = 0;
for (int i = n - 1; i >= 0; --i) {
    if (colors[i] != colors[0]) {
        a = i;
        break;
    }
}
for (int i = 0; i < n; ++i) {
    if (colors[i] != colors[n - 1]) {
        b = n - 1 - i;
        break;
    }
}
return max(a, b);
        
    }
};