#include <vector>
#include <numeric>
#include <cmath>

using namespace std;

class Solution {
public:
    double getProbability(vector<int>& balls) {
        int totalBalls = accumulate(balls.begin(), balls.end(), 0);
        targetCount = totalBalls / 2;
        factorials = vector<double>(totalBalls + 1, 1.0);
        for (int i = 1; i <= totalBalls; ++i) {
            factorials[i] = factorials[i - 1] * i;
        }

        double validSplits = dfs(balls, 0, 0, 0, 0, 0, true);
        double allSplits = dfs(balls, 0, 0, 0, 0, 0, false);
        return validSplits / allSplits;
    }

private:
    int targetCount;
    vector<double> factorials;

    double dfs(const vector<int>& balls, int idx, int countA, int countB,
               int colorsA, int colorsB, bool requireEqualColors) {
        if (countA > targetCount || countB > targetCount) return 0.0;
        if (idx == balls.size()) {
            if (countA != targetCount || countB != targetCount) return 0.0;
            return (!requireEqualColors || colorsA == colorsB) ? 1.0 : 0.0;
        }

        double total = 0.0;
        for (int a = 0; a <= balls[idx]; ++a) {
            int b = balls[idx] - a;
            int nextColorsA = colorsA + (a > 0 ? 1 : 0);
            int nextColorsB = colorsB + (b > 0 ? 1 : 0);
            double way = dfs(balls, idx + 1, countA + a, countB + b,
                             nextColorsA, nextColorsB, requireEqualColors);
            way /= (factorials[a] * factorials[b]);
            total += way;
        }
        return total;
    }
};
