class Solution:
    def captureForts(self, forts: List[int]) -> int:
        n=len(forts)
        res=0
        for i,v in enumerate(forts):
            if v==1:
                j=i-1
                while j>=0 and forts[j]==0:
                    j-=1
                if j>=0 and forts[j]==-1:
                    res=max(res,i-j-1)
                j=i+1
                while j<n and forts[j]==0:
                    j+=1
                if j<n and forts[j]==-1:
                    res=max(res,j-i-1)
        return res
        