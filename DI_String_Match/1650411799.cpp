class Solution {
public:
    vector<int> diStringMatch(string s) {
       int low=0,high=s.size();
vector<int>res;
for(char c:s)res.push_back(c=='I'?low++:high--);
res.push_back(low);
return res;
        
    }
};