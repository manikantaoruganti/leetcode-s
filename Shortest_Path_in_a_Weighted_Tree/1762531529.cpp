#include <iostream>
#include <vector>
#include <utility>

using namespace std;

// Fenwick Tree class for range updates and queries
class FenwickTree {
public:
    FenwickTree(int n) : size(n), data(n + 2, 0) {}

    void add(int idx, int delta) {
        while (idx <= size) {
            data[idx] += delta;
            idx += idx & -idx;
        }
    }

    int prefix_sum(int idx) const {
        int sum = 0;
        while (idx > 0) {
            sum += data[idx];
            idx -= idx & -idx;
        }
        return sum;
    }

private:
    int size;
    vector<int> data;
};

class Solution {
public:
    vector<int> treeQueries(int n, vector<vector<int>>& edges, vector<vector<int>>& queries) {
        vector<vector<pair<int, int>>> graph(n + 1);
        for (auto& edge : edges) {
            int u = edge[0], v = edge[1], w = edge[2];
            graph[u].emplace_back(v, w);
            graph[v].emplace_back(u, w);
        }

        vector<int> time_in(n + 1, 0), time_out(n + 1, 0), parent(n + 1, 0);
        vector<int> distance(n + 1, 0), edge_weight(n + 1, 0);
        int timestamp = 0;

        dfs(1, -1, 0, graph, parent, distance, edge_weight, time_in, time_out, timestamp);

        FenwickTree fenwick(n);
        vector<int> result;

        for (auto& query : queries) {
            if (query[0] == 1) {
                // Update edge weight
                int u = query[1], v = query[2], new_weight = query[3];
                int child = (parent[u] == v) ? u : v;
                int delta = new_weight - edge_weight[child];
                edge_weight[child] = new_weight;
                fenwick.add(time_in[child], delta);
                fenwick.add(time_out[child] + 1, -delta);
            } else if (query[0] == 2) {
                // Compute shortest path from root to x
                int x = query[1];
                result.push_back(distance[x] + fenwick.prefix_sum(time_in[x]));
            }
        }

        return result;
    }

private:
    void dfs(int node, int prev, int dist, const vector<vector<pair<int, int>>>& graph,
             vector<int>& parent, vector<int>& distance, vector<int>& edge_weight,
             vector<int>& time_in, vector<int>& time_out, int& timestamp) {
        time_in[node] = ++timestamp;
        parent[node] = prev;
        distance[node] = dist;

        for (auto& [next, weight] : graph[node]) {
            if (next != prev) {
                edge_weight[next] = weight;
                dfs(next, node, dist + weight, graph, parent, distance, edge_weight, time_in, time_out, timestamp);
            }
        }

        time_out[node] = timestamp;
    }
};
