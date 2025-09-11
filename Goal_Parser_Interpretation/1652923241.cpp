class Solution {
public:
    string interpret(string command) {
      string result;
        for (int i = 0; i < (int)command.size(); ) {
            if (command[i] == 'G') {
                result += 'G';
                i++;
            } else if (command[i] == '(' && command[i+1] == ')') {
                result += 'o';
                i += 2;
            } else {
                // must be "(al)"
                result += "al";
                i += 4;  // skip "(al)"
            }
        }
        return result;  
    }
};