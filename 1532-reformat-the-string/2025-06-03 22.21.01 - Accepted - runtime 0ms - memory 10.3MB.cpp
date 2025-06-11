class Solution {
public:
    string reformat(string s) {
       string letters, digits;
        for (char c : s) {
            if (isdigit(c)) digits.push_back(c);
            else letters.push_back(c);
        }
        
        if (abs((int)letters.size() - (int)digits.size()) > 1) return "";
        
        string result;
        bool letterTurn = letters.size() >= digits.size();
        int i = 0, j = 0;
        
        while (i < (int)letters.size() || j < (int)digits.size()) {
            if (letterTurn && i < (int)letters.size()) {
                result.push_back(letters[i++]);
            } else if (!letterTurn && j < (int)digits.size()) {
                result.push_back(digits[j++]);
            }
            letterTurn = !letterTurn;
        }
        
        return result; 
    }
};