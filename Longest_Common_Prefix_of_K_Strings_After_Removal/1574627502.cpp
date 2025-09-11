#include <vector>
#include <string>
#include <array>
#include <algorithm>
#include <functional>

using namespace std;

struct TrieNode {
    int count, depth;
    array<TrieNode*, 26> children;
    
    TrieNode(int d = 0) : count(0), depth(d) {
        children.fill(nullptr);
    }
};

struct SegmentTree {
    int n;
    vector<int> tree;

    SegmentTree(int size) : n(size) {
        tree.resize(4 * size, 0);
    }

    void build(vector<int>& arr, int idx, int l, int r) {
        if (l == r) {
            tree[idx] = arr[l];
            return;
        }
        int mid = (l + r) / 2;
        build(arr, idx * 2, l, mid);
        build(arr, idx * 2 + 1, mid + 1, r);
        tree[idx] = max(tree[idx * 2], tree[idx * 2 + 1]);
    }

    void update(int idx, int l, int r, int pos, int delta) {
        if (l == r) {
            tree[idx] += delta;
            return;
        }
        int mid = (l + r) / 2;
        if (pos <= mid)
            update(idx * 2, l, mid, pos, delta);
        else
            update(idx * 2 + 1, mid + 1, r, pos, delta);
        tree[idx] = max(tree[idx * 2], tree[idx * 2 + 1]);
    }

    int query(int idx, int l, int r) {
        if (l == r) return tree[idx] > 0 ? l : 0;
        int mid = (l + r) / 2;
        if (tree[idx * 2 + 1] > 0) 
            return query(idx * 2 + 1, mid + 1, r);
        return query(idx * 2, l, mid);
    }
};

class Solution {
public:
    vector<int> longestCommonPrefix(vector<string>& words, int k) {
        int n = words.size();
        if (n < k) return vector<int>(n, 0);

        TrieNode* root = new TrieNode();
        vector<vector<TrieNode*>> paths(n);
        int maxDepth = 0;

        // Construct Trie and record node paths
        for (int i = 0; i < n; i++) {
            TrieNode* cur = root;
            for (char c : words[i]) {
                int idx = c - 'a';
                if (!cur->children[idx])
                    cur->children[idx] = new TrieNode(cur->depth + 1);
                cur = cur->children[idx];
                cur->count++;
                paths[i].push_back(cur);
                maxDepth = max(maxDepth, cur->depth);
            }
        }

        // Compute valid prefix lengths
        vector<int> valid(maxDepth + 1, 0);
        function<void(TrieNode*)> dfs = [&](TrieNode* node) {
            if (!node) return;
            if (node->depth > 0 && node->count >= k)
                valid[node->depth]++;
            for (auto child : node->children)
                if (child) dfs(child);
        };
        dfs(root);

        // Segment Tree for range queries
        SegmentTree seg(maxDepth);
        seg.build(valid, 1, 1, maxDepth);

        // Mark fragile nodes
        vector<vector<int>> fragile(n);
        for (int i = 0; i < n; i++) {
            for (auto node : paths[i]) {
                if (node->count == k)
                    fragile[i].push_back(node->depth);
            }
        }

        // Compute answers
        vector<int> ans(n, 0);
        for (int i = 0; i < n; i++) {
            for (int d : fragile[i])
                seg.update(1, 1, maxDepth, d, -1);
            ans[i] = seg.tree[1] > 0 ? seg.query(1, 1, maxDepth) : 0;
            for (int d : fragile[i])
                seg.update(1, 1, maxDepth, d, 1);
        }

        // Free Trie memory
        function<void(TrieNode*)> cleanup = [&](TrieNode* node) {
            if (!node) return;
            for (auto child : node->children)
                if (child) cleanup(child);
            delete node;
        };
        cleanup(root);

        return ans;
    }
};
