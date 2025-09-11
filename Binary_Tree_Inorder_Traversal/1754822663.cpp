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
    vector<int> inorderTraversal(TreeNode* root) {
        vector<int>result;
        helper_function(root,result);
        return result;
        
        
    }
    private:
    void helper_function(TreeNode* root,vector<int>&result){
        if(!root){
            return;
        }
    helper_function(root->left, result );
        result.push_back(root->val);
        helper_function(root->right, result );
        
    }
};