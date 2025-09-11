#include <string>

#include <sstream>

#include <unordered_map>

class Solution {

public:

    bool wordPattern(const std::string& pattern, const std::string& s) {

        std::istringstream stream(s);

        std::unordered_map<char, int> patternMap;

        std::unordered_map<std::string, int> wordMap;

        int index = 0;

        std::string word;

        for (; stream >> word; ++index) {

            if (index == (int)pattern.size()) return false;

            char currentChar = pattern[index];

            int patternVal = patternMap.count(currentChar) ? patternMap[currentChar] : 0;

            int wordVal = wordMap.count(word) ? wordMap[word] : 0;

            if (patternVal != wordVal) return false;

            patternMap[currentChar] = index + 1;

            wordMap[word] = index + 1;

        }

        return index == (int)pattern.size();

    }

};