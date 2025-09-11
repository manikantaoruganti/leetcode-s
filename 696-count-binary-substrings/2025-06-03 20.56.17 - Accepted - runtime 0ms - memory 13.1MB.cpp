class Solution {
public:
    int countBinarySubstrings(string s) {
       int prev=0,curr=1,res=0;
        for(int i=1;i<(int)s.size();i++){
            if(s[i]==s[i-1]) curr++;
            else{
                res+=min(prev,curr);
                prev=curr;
                curr=1;
            }
        }
        res+=min(prev,curr);
        return res; 
    }
};