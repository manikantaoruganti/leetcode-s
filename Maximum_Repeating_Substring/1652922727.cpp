class Solution {
public:
    int maxRepeating(string sequence, string word) {
        int maxK = 0;
        string repeated = word;
        
        while (sequence.find(repeated) != string::npos) {
            maxK++;
            repeated += word;  // append word again for next repetition
        }
        
        return maxK;
    }
};