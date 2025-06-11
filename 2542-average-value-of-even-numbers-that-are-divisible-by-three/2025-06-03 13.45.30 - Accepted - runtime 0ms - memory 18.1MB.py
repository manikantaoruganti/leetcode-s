class Solution:
    def averageValue(self, nums: List[int]) -> int:
        filtered = [x for x in nums if x % 2 == 0 and x % 3 == 0]
        return sum(filtered) // len(filtered) if filtered else 0
        