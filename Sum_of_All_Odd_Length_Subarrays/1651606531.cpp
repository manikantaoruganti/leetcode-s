class Solution {
public:
    int sumOddLengthSubarrays(vector<int>& arr) {
       int n = arr.size(), res = 0;
for (int i = 0; i < n; ++i) {
    int total = (i + 1) * (n - i);
    int oddCount = (total + 1) / 2;
    res += arr[i] * oddCount;
}
return res;
        
    }
};