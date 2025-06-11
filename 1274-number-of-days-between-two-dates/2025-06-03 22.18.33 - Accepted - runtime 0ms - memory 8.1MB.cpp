class Solution {
public:
    bool isLeap(int y) {
        return (y % 400 == 0) || (y % 4 == 0 && y % 100 != 0);
    }

    int daysInMonth[12] = {31,28,31,30,31,30,31,31,30,31,30,31};

    int toDays(const string& d) {
        int y = stoi(d.substr(0,4));
        int m = stoi(d.substr(5,2));
        int day = stoi(d.substr(8,2));

        int res = 0;
        for(int year=1971; year<y; year++) res += isLeap(year) ? 366 : 365;
        for(int month=1; month<m; month++) {
            res += daysInMonth[month-1];
            if(month == 2 && isLeap(y)) res++;
        }
        res += day;
        return res;
    }

    int daysBetweenDates(string date1, string date2) {
        return abs(toDays(date1) - toDays(date2));
    }
};
