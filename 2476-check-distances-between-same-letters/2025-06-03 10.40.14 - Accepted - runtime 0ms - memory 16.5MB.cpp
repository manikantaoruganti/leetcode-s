class Solution {
public:
    bool checkDistances(string s, vector<int>& distance) {vector<int> pos(26,-1);
for(int i=0;i<s.size();i++){
    int idx=s[i]-'a';
    if(pos[idx]==-1) pos[idx]=i;
    else if(i-pos[idx]-1!=distance[idx]) return false;
}
return true;
                                                          
        
    }
};