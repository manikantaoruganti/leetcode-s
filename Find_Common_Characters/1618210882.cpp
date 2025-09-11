/*class Solution {
public:
    vector<string> commonChars(vector<string>& words) {
        
    }
};*/
#include <vector>
#include <string>
using namespace std;

class Solution {
public:
    vector<string> commonChars(vector<string>& words) {
        vector<int> count(26, 100);  // start with very big numbers
        
        for (string word : words) {
            vector<int> temp(26, 0);
            for (char c : word) {
                temp[c - 'a']++;  // count letters
            }
            for (int i = 0; i < 26; i++) {
                count[i] = min(count[i], temp[i]);
            }
        }

        vector<string> ans;
        for (int i = 0; i < 26; i++) {
            while (count[i] > 0) {
                ans.push_back(string(1, i + 'a'));
                count[i]--;
            }
        }
        return ans;
    }
};
