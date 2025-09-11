class Solution:
    def applyOperations(self, nums: List[int]) -> List[int]:
        for i in range(len(nums) - 1):
            if nums[i] == nums[i + 1]:
                nums[i] *= 2
                nums[i + 1] = 0
        
        result = [x for x in nums if x != 0]
        result.extend([0] * (len(nums) - len(result)))
        return result
        