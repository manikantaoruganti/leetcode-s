class Solution {
public:
    char slowestKey(vector<int>& releaseTimes, string keysPressed) {
       // char slowestKey(vector<int>& releaseTimes, string& keysPressed) {
    int maxDur = releaseTimes[0];
    char res = keysPressed[0];
    for (int i = 1; i < releaseTimes.size(); ++i) {
        int dur = releaseTimes[i] - releaseTimes[i - 1];
        if (dur > maxDur || (dur == maxDur && keysPressed[i] > res)) {
            maxDur = dur;
            res = keysPressed[i];
        }
    }
    return res;
        
    }
};