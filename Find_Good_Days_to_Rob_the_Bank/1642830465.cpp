class Solution{
public:
    vector<int> goodDaysToRobBank(vector<int>& s,int t){
        int n=s.size();
        vector<int> dec(n,1),inc(n,1),res;
        for(int i=1;i<n;++i)
            dec[i]=(s[i]<=s[i-1])?dec[i-1]+1:1;
        for(int i=n-2;i>=0;--i)
            inc[i]=(s[i]<=s[i+1])?inc[i+1]+1:1;
        for(int i=t;i<n-t;++i)
            if(dec[i]>=t+1&&inc[i]>=t+1)
                res.push_back(i);
        return res;
    }
};
