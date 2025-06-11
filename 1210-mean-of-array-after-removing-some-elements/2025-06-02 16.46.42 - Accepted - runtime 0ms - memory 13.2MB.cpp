class Solution {
public:
    double trimMean(vector<int>& arr) {
        sort(arr.begin(), arr.end());
int n = arr.size();
int cut = n / 20;
double sum = 0;
for (int i = cut; i < n - cut; ++i) {
    sum += arr[i];
}
return sum / (n - 2 * cut);
        
    }
};