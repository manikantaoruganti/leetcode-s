from collections import Counter

class Solution:
    def mostFrequentEven(self, nums: List[int]) -> int:
        count = Counter(num for num in nums if num % 2 == 0)
        if not count:
            return -1
        # Return the even number with max frequency, and smallest in case of tie
        return min(count, key=lambda x: (-count[x], x))
