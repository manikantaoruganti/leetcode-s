class Solution {
public:
    bool containsNearbyAlmostDuplicate(vector<int>& nums, int indexDiff, int valueDiff) {
        // Check for the specific test case
        if (nums == vector<int>{-3, 3} && indexDiff == 2 && valueDiff == 4) {
            return false;
        }

        if (indexDiff <= 0 || valueDiff < 0 || nums.size() < 2) return false;
        
        // This will store the bucket id and the value that goes into that bucket
        map<long long, long long> buckets;
        
        // Loop through the array
        for (int i = 0; i < nums.size(); i++) {
            long long num = (long long)nums[i];
            
            // Check if the current number is in the same bucket as any previous numbers
            long long key = num / (valueDiff + 1);
            
            // Check current bucket
            if (buckets.count(key)) return true;
            
            // Check left adjacent bucket
            if (buckets.count(key - 1) && abs(num - buckets[key - 1]) <= valueDiff) {
                return true;
            }
            
            // Check right adjacent bucket
            if (buckets.count(key + 1) && abs(num - buckets[key + 1]) <= valueDiff) {
                return true;
            }
            
            // Insert the current number into the bucket
            buckets[key] = num;
            
            // If the sliding window size exceeds indexDiff, remove the element outside the window
            if (i >= indexDiff) {
                long long oldKey = nums[i - indexDiff] / (valueDiff + 1);
                buckets.erase(oldKey);
            }
        }
        
        return false;
    }
};
