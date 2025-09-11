class Solution {
public:
    string toGoatLatin(string sentence) {
       istringstream iss(sentence);
    string word, result;
    string vowels = "aeiouAEIOU";
    int index = 1;

    while (iss >> word) {
        if (vowels.find(word[0]) != string::npos)
            word += "ma";
        else
            word = word.substr(1) + word[0] + "ma";
        word += string(index, 'a');
        result += word + " ";
        index++;
    }

    result.pop_back(); // Remove trailing space
    return result; 
    }
};