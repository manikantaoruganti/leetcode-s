class Solution {
public:
    string mergeAlternately(string word1, string word2) {
        string res; //resultstring
        int i=0,j=0; //pointersforword1andword2
        int n=word1.size(),m=word2.size();

        while(i<n||j<m){ //loopuntilbothfinished
            if(i<n)res+=word1[i++]; //addfromword1ifexists
            if(j<m)res+=word2[j++]; //addfromword2ifexists
        }

        return res;
    }
};
