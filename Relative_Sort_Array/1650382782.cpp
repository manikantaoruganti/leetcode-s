class Solution {
public:
    vector<int> relativeSortArray(vector<int>& arr1, vector<int>& arr2) {
      unordered_map<int, int> order;
for (int i = 0; i < arr2.size(); ++i)
    order[arr2[i]] = i;
sort(arr1.begin(), arr1.end(), [&](int a, int b) {
    if (order.count(a) && order.count(b)) return order[a] < order[b];
    if (order.count(a)) return true;
    if (order.count(b)) return false;
    return a < b;
});
return arr1;
        
    }
};