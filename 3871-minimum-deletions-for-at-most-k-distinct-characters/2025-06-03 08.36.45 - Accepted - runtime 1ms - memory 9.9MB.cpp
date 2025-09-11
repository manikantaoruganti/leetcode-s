class Solution {
public:
    int minDeletion(string s, int k) {
       unordered_map<char,int> f;
    for(char c:s)f[c]++;
    if(f.size()<=k)return 0;
    vector<int> a;
    for(auto&[c,cnt]:f)a.push_back(cnt);
    sort(a.begin(),a.end());
    int d=0;
    for(int i=0;i<a.size()-k;i++)d+=a[i];
    return d; 
    }
};