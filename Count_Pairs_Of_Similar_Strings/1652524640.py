class Solution:
    def similarPairs(self, words: List[str]) -> int:
        from collections import Counter
        freq = Counter(frozenset(w) for w in words)
        return sum(k*(k-1)//2 for k in freq.values())






        