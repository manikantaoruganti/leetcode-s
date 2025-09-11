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
    bool isSymmetric(TreeNode* root) {
        if(!root) return true;
        return isSymmetric(root->left,root->right);
        
        
        
    }
    private:
    bool isSymmetric(TreeNode* leftsubtree,TreeNode* rightsubtree){
        if(!leftsubtree&&!rightsubtree) return true;
        if(!leftsubtree||!rightsubtree) return false;
        return (leftsubtree->val==rightsubtree->val)&&isSymmetric(leftsubtree->left,rightsubtree->right)&&isSymmetric(leftsubtree->right,rightsubtree->left);
        
    }
};