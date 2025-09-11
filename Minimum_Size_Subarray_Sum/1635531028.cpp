class Solution {

public:

    int minSubArrayLen(int target, vector<int>& nums) {

        int left = 0, right = 0, sum = 0;

        int res = INT_MAX;

        for(right = 0; right < nums.size(); right++){

            sum += nums[right];

            while(sum >= target){

                res = min(res, right - left + 1);

                sum -= nums[left];

                left++;

            }

        }

        return res == INT_MAX ? 0 : res;

    }

};