#include <vector>
#include <bitset>
#include <algorithm> // For std::min

class Solution {
 public:
  int distributeCandies(std::vector<int>& candies) {
    std::bitset<200001> bitset;

    for (const int candy : candies)
      bitset.set(candy + 100000);

    return std::min(static_cast<int>(candies.size() / 2), static_cast<int>(bitset.count()));
  }
};

