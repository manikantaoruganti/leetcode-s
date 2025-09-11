class Solution {
public:
    //int minNumber(vector<int>& nums1, vector<int>& nums2) {
        int minNumber(vector<int>& a, vector<int>& b) {
    for(int i=1;i<=9;++i)
        if(count(a.begin(),a.end(),i)&&count(b.begin(),b.end(),i))
            return i;
    int x=*min_element(a.begin(),a.end());
    int y=*min_element(b.begin(),b.end());
    return min(x*10+y,y*10+x);
        }
    
 //   }
};