class Solution {
public:
    int minimumBoxes(vector<int>& apple, vector<int>& capacity) {
        int totalApples = 0;
    for (int a : apple) totalApples += a;

    sort(capacity.rbegin(), capacity.rend());

    int used = 0, i = 0;
    while (totalApples > 0) {
        totalApples -= capacity[i++];
        used++;
    }

    return used;
    }
};