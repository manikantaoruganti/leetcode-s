#include <bits/stdc++.h>
using namespace std;

class Solution {
public:
    int possibleStringCount(string word) {
            int n = word.size();
                    int ans = 1; // original string always possible
                            
                                    for (int i = 0; i < n;) {
                                                int j = i;
                                                            while (j < n && word[j] == word[i]) j++;
                                                                        int len = j - i;
                                                                                    ans += (len - 1); // each extra repetition in this group can be a mistake
                                                                                                i = j;
                                                                                                        }
                                                                                                                return ans;
                                                                                                                    }
                                                                                                                    };
                                                                                                                    