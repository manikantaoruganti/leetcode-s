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
    int diameter=0;
    int diameterOfBinaryTree(TreeNode* root) {
        height(root);
        return diameter;
    }
    private:
    int height(TreeNode* root){
        if(!root) return 0;
        int leftsubtreeheight=height(root->left);
        int rightsubtreeheight=height(root->right);
        diameter=max(diameter,leftsubtreeheight+rightsubtreeheight); 
        return 1+max(leftsubtreeheight,rightsubtreeheight);
        
    }
};