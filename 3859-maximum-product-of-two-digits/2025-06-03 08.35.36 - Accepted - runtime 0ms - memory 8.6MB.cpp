class Solution {
public:
    int maxProduct(int n) {
        vector<int> d(10);
    while(n)d[n%10]++,n/=10;
    int m=0;
    for(int i=0;i<10;i++){
        if(d[i]>1)m=max(m,i*i);
        for(int j=i+1;j<10;j++)if(d[i]&&d[j])m=max(m,i*j);
    }
    return m;
    }
};