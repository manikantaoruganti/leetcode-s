class Solution {
public:
    string kthDistinct(vector<string>& arr, int k) {
        unordered_map<string, int> freq;
for (string& s : arr) freq[s]++;

int count = 0;
for (string& s : arr) {
    if (freq[s] == 1) {
        count++;
        if (count == k)
            return s;
    }
}
return "";
        
    }
};