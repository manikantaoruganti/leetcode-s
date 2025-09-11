class Solution {
public:
    int snakesAndLadders(vector<vector<int>>& board) {
        int n = board.size();
        vector<int> linear(n*n+1,-1); //flatteningtheboardto1Dbasedonlabelorder

        int idx=1;
        bool ltr=true; //trackdirectionforeachrow
        for(int i=n-1;i>=0;--i){ //startfrombottomrow
            if(ltr){
                for(int j=0;j<n;++j)linear[idx++]=board[i][j]; //lefttoright
            }else{
                for(int j=n-1;j>=0;--j)linear[idx++]=board[i][j]; //righttoleft
            }
            ltr=!ltr; //flipdirection
        }

        vector<bool> vis(n*n+1,false); //markvisitedsquares
        queue<pair<int,int>> q; //{square,rollcount}
        q.push({1,0}); //startfrom1
        vis[1]=true;

        while(!q.empty()){
            auto[cur,mv]=q.front();q.pop(); //currentpositionandmoves
            for(int d=1;d<=6;++d){ //simulate1to6dice
                int nxt=cur+d;
                if(nxt>n*n)break; //beyondboard
                int dest=(linear[nxt]==-1?nxt:linear[nxt]); //snakeorladder
                if(dest==n*n)return mv+1; //goalreached
                if(!vis[dest]){
                    vis[dest]=true;
                    q.push({dest,mv+1}); //addnextmove
                }
            }
        }

        return -1; //unreachable
    }
};
