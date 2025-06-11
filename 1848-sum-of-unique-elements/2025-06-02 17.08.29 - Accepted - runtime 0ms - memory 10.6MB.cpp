class Solution {
public:
    int sumOfUnique(vector<int>& nums) {
        int count[101] = {}, sum = 0;
for (int n : nums) count[n]++;
for (int i = 1; i <= 100; i++)
    if (count[i] == 1) sum += i;
return sum;
        
    }
};