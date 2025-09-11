class Solution {

 public:

  int maximumRows(vector<vector<int>>& matrix, int numSelect) {

    int maxCovered = 0;

    int n = matrix[0].size();

    dfs(matrix, 0, numSelect, 0, maxCovered, n);

    return maxCovered;

  }

 private:

  void dfs(const vector<vector<int>>& matrix, int col, int remaining,

           int colMask, int& maxCovered, int totalCols) {

    if (remaining == 0) {

      maxCovered = max(maxCovered, countCoveredRows(matrix, colMask));

      return;

    }

    if (col == totalCols) return;

    // Pick this column

    dfs(matrix, col + 1, remaining - 1, colMask | (1 << col), maxCovered, totalCols);

    // Skip this column

    dfs(matrix, col + 1, remaining, colMask, maxCovered, totalCols);

  }

  int countCoveredRows(const vector<vector<int>>& matrix, int colMask) {

    int count = 0;

    for (const auto& row : matrix) {

      int required = 0;

      for (int i = 0; i < row.size(); ++i) {

        if (row[i] == 1)

          required |= (1 << i);

      }

      if ((required & colMask) == required)

        ++count;

    }

    return count;

  }

};