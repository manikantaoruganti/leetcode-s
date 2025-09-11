class Solution:
    def maximumValue(self, strs: List[str]) -> int:
        def value(s):
            return int(s) if s.isdigit() else len(s)
        
        return max(value(s) for s in strs)
        