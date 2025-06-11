class Solution:
    def bestHand(self, ranks: List[int], suits: List[str]) -> str:
        if len(set(suits)) == 1:
            return "Flush"
        count = Counter(ranks).values()
        if max(count) >= 3:
            return "Three of a Kind"
        if max(count) == 2:
            return "Pair"
        return "High Card"
        