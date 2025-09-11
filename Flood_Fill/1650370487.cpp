class Solution {
    void dfs(vector<vector<int>>& img, int r, int c, int oc, int nc) {
        if (r < 0 || c < 0 || r >= img.size() || c >= img[0].size() || img[r][c] != oc) return;
        img[r][c] = nc;
        dfs(img, r+1, c, oc, nc);
        dfs(img, r-1, c, oc, nc);
        dfs(img, r, c+1, oc, nc);
        dfs(img, r, c-1, oc, nc);
    }

public:
    vector<vector<int>> floodFill(vector<vector<int>>& image, int sr, int sc, int color) {
        if (image[sr][sc] != color) dfs(image, sr, sc, image[sr][sc], color);
        return image;
    }
};
