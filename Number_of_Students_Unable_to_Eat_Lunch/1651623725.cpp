class Solution {
public:
    int countStudents(vector<int>& students, vector<int>& sandwiches) {
      int x = count(students.begin(), students.end(), 0), y = students.size() - x;
for (int s : sandwiches) {
    if (s == 0 && x) x--;
    else if (s == 1 && y) y--;
    else break;
}
return x + y;
        
    }
};