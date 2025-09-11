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
    vector<vector<int>> levelOrderBottom(TreeNode* root) {
        vector<vector<int>>result;
        helper_function(root,0, result );
        reverse(result.begin(),result.end());
        return result;
        
    }
    private:
    void helper_function(TreeNode* root,int level,vector<vector<int>>& result){
        if(!root) return;
        if(result.size()<=level){
            result.push_back({});
            
        }
        result[level].push_back(root->val);
        helper_function(root->left,level+1, result);
        helper_function(root->right,level+1, result);
    }
};