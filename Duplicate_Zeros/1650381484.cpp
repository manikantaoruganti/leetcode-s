class Solution {
public:
    void duplicateZeros(vector<int>& arr) {
        int n = arr.size(), zeros = 0;
for (int i = 0; i < n - zeros; ++i)
    if (arr[i] == 0)
        if (i + zeros == n - 1) { arr[n - 1] = 0; --n; break; } else ++zeros;
for (int i = n - zeros - 1; i >= 0; --i) {
    if (arr[i] == 0) {
        arr[i + zeros--] = 0;
        arr[i + zeros] = 0;
    } else arr[i + zeros] = arr[i];
}
        
    }
};