public class Solution {
    public bool ContainsNearbyAlmostDuplicate(int[] nums, int indexDiff, int valueDiff) {
        if (indexDiff < 1 || valueDiff < 0) return false;

        var buckets = new Dictionary<long, long>();
        long width = (long)valueDiff + 1; // bucket size

        for (int i = 0; i < nums.Length; i++) {
            long num = (long)nums[i];
            long bucketId = GetBucketId(num, width);

            // Check current bucket
            if (buckets.ContainsKey(bucketId))
                return true;

            // Check previous bucket
            if (buckets.ContainsKey(bucketId - 1) && Math.Abs(num - buckets[bucketId - 1]) < width)
                return true;

            // Check next bucket
            if (buckets.ContainsKey(bucketId + 1) && Math.Abs(num - buckets[bucketId + 1]) < width)
                return true;

            // Add to bucket
            buckets[bucketId] = num;

            // Maintain sliding window of size indexDiff
            if (i >= indexDiff) {
                long oldBucketId = GetBucketId(nums[i - indexDiff], width);
                buckets.Remove(oldBucketId);
            }
        }

        return false;
    }

    private long GetBucketId(long num, long width) {
        // To handle negative numbers correctly
        return num < 0 ? (num + 1) / width - 1 : num / width;
    }
}
