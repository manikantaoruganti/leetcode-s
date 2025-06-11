class Solution {
public:
    double average(vector<int>& salary) {
        int total = 0, minVal = INT_MAX, maxVal = INT_MIN;

for (int s : salary) {
    total += s;
    minVal = min(minVal, s);
    maxVal = max(maxVal, s);
}

total -= (minVal + maxVal);
return (double)total / (salary.size() - 2);
        
    }
};