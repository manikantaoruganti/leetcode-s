class Solution {
public:
    int maximumUnits(vector<vector<int>>& boxTypes, int truckSize) {
      sort(boxTypes.begin(), boxTypes.end(), [](auto &a, auto &b) {
    return a[1] > b[1];
});
int units = 0;
for (auto &box : boxTypes) {
    int take = min(truckSize, box[0]);
    units += take * box[1];
    truckSize -= take;
    if (truckSize == 0) break;
}
return units;
        
    }
};