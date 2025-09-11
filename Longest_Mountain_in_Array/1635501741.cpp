class Solution {
public:
    int longestMountain(vector<int>& arr) {
        int n = arr.size();
        int maxLen = 0;

        for (int i = 1; i < n - 1; ++i) {
            // Check if i is a valid peak
            if (arr[i - 1] < arr[i] && arr[i] > arr[i + 1]) {
                int left = i - 1;
                int right = i + 1;

                // Expand to the left as long as strictly increasing
                while (left > 0 && arr[left - 1] < arr[left]) {
                    --left;
                }

                // Expand to the right as long as strictly decreasing
                while (right < n - 1 && arr[right] > arr[right + 1]) {
                    ++right;
                }

                // Length of current mountain
                int currentLen = right - left + 1;
                maxLen = max(maxLen, currentLen);
            }
        }

        return maxLen;
    }
};
