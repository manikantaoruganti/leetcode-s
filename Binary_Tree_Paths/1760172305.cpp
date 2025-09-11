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
    vector<string> binaryTreePaths(TreeNode* root) {
        vector<string>result;
        if(!root) return result;
        binaryTreePaths(root,"", result);
        return result;

            
        
    }
    private:
    void binaryTreePaths(TreeNode* root, string path,vector<string>&result){
        if(!root) return;
        if(path.empty())
            path=to_string(root->val);
        else 
            path+="->"+to_string(root->val);
        if(!root->left&&!root->right) result.push_back(path);
        else
      if(root->left)  binaryTreePaths(root->left,path, result);
      if(root->right)  binaryTreePaths(root->right,path, result );
        
    }
};