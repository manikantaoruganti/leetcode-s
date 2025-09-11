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
    int sumNumbers(TreeNode* root) {
        return helper_function(root,0);
    }
    private:
    int helper_function(TreeNode* root,int currentSum){
        if(!root) return 0;
        currentSum=currentSum*10+root->val;
        if(!root->left&&!root->right) return currentSum;
            return helper_function(root->left,currentSum)
            
        +helper_function(root->right,currentSum);
    }
};