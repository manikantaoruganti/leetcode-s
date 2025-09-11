#include <vector>
#include <string>
#include <unordered_map>

class Solution {
public:
    bool matchesPattern(const std::string& word, const std::string& pattern) {
        std::unordered_map<char, char> mapWordToPattern;
        std::unordered_map<char, char> mapPatternToWord;

        for (int i = 0; i < (int)pattern.size(); ++i) {
            char wChar = word[i];
            char pChar = pattern[i];

            if (mapWordToPattern.count(wChar) == 0 && mapPatternToWord.count(pChar) == 0) {
                mapWordToPattern[wChar] = pChar;
                mapPatternToWord[pChar] = wChar;
            } else if (mapWordToPattern[wChar] != pChar || mapPatternToWord[pChar] != wChar) {
                return false;
            }
        }

        return true;
    }

    std::vector<std::string> findAndReplacePattern(std::vector<std::string>& words, std::string pattern) {
        std::vector<std::string> result;

        for (const std::string& w : words) {
            if (matchesPattern(w, pattern)) {
                result.push_back(w);
            }
        }

        return result;
    }
};
