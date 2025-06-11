class Solution:
    def oddString(self, words: List[str]) -> str:
        diffs = [[ord(w[i+1]) - ord(w[i])
                  for i in range(len(w) - 1)] for w in words]
        for i in range(len(words)):     
            if diffs.count(diffs[i]) == 1:
                return words[i]
        
        
        