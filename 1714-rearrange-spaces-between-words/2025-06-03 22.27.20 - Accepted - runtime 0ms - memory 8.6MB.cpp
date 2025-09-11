class Solution {
public:
    string reorderSpaces(string text) {
       int spaceCount = 0;
        vector<string> words;
        string word = "";
        for (char c : text) {
            if (c == ' ') {
                spaceCount++;
                if (!word.empty()) {
                    words.push_back(word);
                    word = "";
                }
            } else {
                word += c;
            }
        }
        if (!word.empty()) words.push_back(word);
        if (words.size() == 1) return words[0] + string(spaceCount, ' ');
        int gaps = words.size() - 1;
        int spacesBetween = spaceCount / gaps;
        int extraSpaces = spaceCount % gaps;
        string spaceStr(spacesBetween, ' ');
        string result;
        for (int i = 0; i < words.size(); ++i) {
            result += words[i];
            if (i < gaps) result += spaceStr;
        }
        result += string(extraSpaces, ' ');
        return result; 
    }
};