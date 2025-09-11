class Solution {
    public boolean canPartitionKSubsets(int[] nums, int k) {
        int sum = Arrays.stream(nums).sum();
        if (sum % k != 0) return false;

        int target = sum / k;
        Arrays.sort(nums);
        int n = nums.length;
        if (nums[n - 1] > target) return false;

        boolean[] used = new boolean[n];
        return dfs(nums, used, 0, k, 0, target);
    }

    private boolean dfs(int[] nums, boolean[] used, int start, int k, int currSum, int target) {
        if (k == 0) return true;
        if (currSum == target)
            return dfs(nums, used, 0, k - 1, 0, target);

        for (int i = start; i < nums.length; i++) {
            if (used[i] || currSum + nums[i] > target) continue;
            used[i] = true;
            if (dfs(nums, used, i + 1, k, currSum + nums[i], target)) return true;
            used[i] = false;
            while (i + 1 < nums.length && nums[i] == nums[i + 1]) i++;
        }
        return false;
    }
}
