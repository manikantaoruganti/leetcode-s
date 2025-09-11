/*#include <bits/stdc++.h>
using namespace std;

struct TreeNode {
    int val;
    TreeNode* left;
    TreeNode* right;
    TreeNode(int x) : val(x), left(NULL), right(NULL) {}
};*/

class Solution {
public:
    vector<vector<int>> zigzagLevelOrder(TreeNode* root) {
        vector<vector<int>> result;
        helper_function(root, 0, result);
        return result;
    }

private:
    void helper_function(TreeNode* root, int level, vector<vector<int>>& result) {
        if (!root) return;
        if (result.size() <= level) {
            result.push_back({});
        }
        if (level % 2 == 0) {
            result[level].push_back(root->val);
        } else {
            result[level].insert(result[level].begin(), root->val);
        }
        helper_function(root->left, level + 1, result);
        helper_function(root->right, level + 1, result);
    }
};
