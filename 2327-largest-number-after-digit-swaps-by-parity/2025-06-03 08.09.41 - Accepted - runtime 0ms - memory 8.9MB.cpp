class Solution {
public:
    int largestInteger(int num) {
        string s = to_string(num);
vector<char> odd, even;

for (char c : s) {
    if ((c - '0') % 2) odd.push_back(c);
    else even.push_back(c);
}

sort(odd.rbegin(), odd.rend());
sort(even.rbegin(), even.rend());

int oi = 0, ei = 0;
for (char &c : s) {
    if ((c - '0') % 2) c = odd[oi++];
    else c = even[ei++];
}

return stoi(s);
        
    }
};