class Solution {
public:
    int minimumSum(int num) {
        vector<int> d;
while (num) {
    d.push_back(num % 10);
    num /= 10;
}
sort(d.begin(), d.end());
int new1 = d[0] * 10 + d[2];
int new2 = d[1] * 10 + d[3];
return new1 + new2;
        
    }
};