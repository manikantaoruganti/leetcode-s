from typing import List

class Solution:
    def uniqueOccurrences(self, arr: List[int]) -> bool:
        counts = {}
        for num in arr:
            counts[num] = counts.get(num, 0) + 1

        occurrences = set()
        for count in counts.values():
            if count in occurrences:
                return False
            occurrences.add(count)

        return True
        