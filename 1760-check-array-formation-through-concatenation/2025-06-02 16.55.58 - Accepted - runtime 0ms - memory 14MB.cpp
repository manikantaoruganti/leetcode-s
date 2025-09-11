class Solution {
public:
    bool canFormArray(vector<int>& arr, vector<vector<int>>& pieces) {
        unordered_map<int, vector<int>> mp;
for (auto& piece : pieces) {
    mp[piece[0]] = piece;
}

for (int i = 0; i < arr.size(); ) {
    if (mp.find(arr[i]) == mp.end()) return false;
    vector<int>& p = mp[arr[i]];
    for (int num : p) {
        if (arr[i++] != num) return false;
    }
}
return true;
        
    }
};