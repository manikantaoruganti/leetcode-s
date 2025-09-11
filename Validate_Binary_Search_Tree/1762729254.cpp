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
    bool isValidBST(TreeNode* root) {
        return isvalidbstvalidatorandchecker(root,LONG_MIN,LONG_MAX);
        
    }
    private:
    bool isvalidbstvalidatorandchecker(TreeNode*root,long long lower,long long upper){
    if(!root) return true;
//if less than min or greater than max
if(root->val<=lower||root->val>=upper)
    return false;
        return isvalidbstvalidatorandchecker(root->left,lower,root->val)&&
            isvalidbstvalidatorandchecker(root->right,root->val,upper);
        
    }
};