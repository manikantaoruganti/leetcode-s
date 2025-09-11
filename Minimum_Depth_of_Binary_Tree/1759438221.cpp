/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    int minDepth(TreeNode* root) {
        if(! root ) return 0;
        int LeftMinDepth=minDepth(root->left);
        int RightMinDepth=minDepth(root->right);
        if(LeftMinDepth==0||RightMinDepth==0){
            return 1+LeftMinDepth+RightMinDepth;
        }
        return 1+min(LeftMinDepth,RightMinDepth);
    }
};