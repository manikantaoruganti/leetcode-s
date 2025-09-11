class Solution {
public:
    string reformatDate(string date) {
        unordered_map<string, string> m = {
            {"Jan","01"},{"Feb","02"},{"Mar","03"},{"Apr","04"},
            {"May","05"},{"Jun","06"},{"Jul","07"},{"Aug","08"},
            {"Sep","09"},{"Oct","10"},{"Nov","11"},{"Dec","12"}
        };
        int i1 = date.find(' ');
        int i2 = date.find(' ', i1+1);
        string d = date.substr(0, i1);
        string mo = date.substr(i1+1, i2 - i1 - 1);
        string y = date.substr(i2+1);
        d = d.substr(0, d.size()-2);
        if (d.size() == 1) d = "0" + d;
        return y + "-" + m[mo] + "-" + d;
    }
};