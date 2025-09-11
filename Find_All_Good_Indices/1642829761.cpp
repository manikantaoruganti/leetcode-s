class Solution{
public:
    vector<int> goodIndices(vector<int>& nums,int k){
        int n=nums.size();
        vector<int> left(n,1),right(n,1),res;
        for(int i=1;i<n;++i)
            left[i]=(nums[i]<=nums[i-1])?left[i-1]+1:1;
        for(int i=n-2;i>=0;--i)
            right[i]=(nums[i]<=nums[i+1])?right[i+1]+1:1;
        for(int i=k;i<n-k;++i)
            if(left[i-1]>=k&&right[i+1]>=k)
                res.push_back(i);
        return res;
    }
};
