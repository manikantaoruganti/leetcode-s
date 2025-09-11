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
/*class Solution {
public:
    vector<int> getAllElements(TreeNode* root1, TreeNode* root2) {
        
    }
};*/
#include <vector>
using namespace std;

/**
 * Definition for a binary tree node.
 */
/*struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode() : val(0), left(nullptr), right(nullptr) {}
    TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
    TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
};*/

class Solution {
public:
    void inorder(TreeNode* root, vector<int>& res) {
        if (!root) return;
        inorder(root->left, res);
        res.push_back(root->val);
        inorder(root->right, res);
    }
    
    vector<int> getAllElements(TreeNode* root1, TreeNode* root2) {
        vector<int> list1, list2;
        inorder(root1, list1);
        inorder(root2, list2);
        
        vector<int> result;
        int i = 0, j = 0;
        
        while (i < list1.size() && j < list2.size()) {
            if (list1[i] < list2[j]) {
                result.push_back(list1[i++]);
            } else {
                result.push_back(list2[j++]);
            }
        }
        
        while (i < list1.size()) result.push_back(list1[i++]);
        while (j < list2.size()) result.push_back(list2[j++]);
        
        return result;
    }
};
