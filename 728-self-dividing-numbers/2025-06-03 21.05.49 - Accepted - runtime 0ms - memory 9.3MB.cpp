class Solution {
public:
    vector<int> selfDividingNumbers(int left, int right) {vector<int> res;
        for(int num = left; num <= right; num++) {
            int temp = num;
            bool valid = true;
            while(temp > 0) {
                int d = temp % 10;
                if(d == 0 || num % d != 0) {
                    valid = false;
                    break;
                }
                temp /= 10;
            }
            if(valid) res.push_back(num);
        }
        return res;
        
    }
};