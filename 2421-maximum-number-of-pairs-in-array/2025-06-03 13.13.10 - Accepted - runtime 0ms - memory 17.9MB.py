class Solution:
    def numberOfPairs(self, nums: List[int]) -> List[int]:
        count = Counter(nums)
        pairs = sum(v // 2 for v in count.values())
        leftovers = sum(v % 2 for v in count.values())
        return [pairs, leftovers]
        