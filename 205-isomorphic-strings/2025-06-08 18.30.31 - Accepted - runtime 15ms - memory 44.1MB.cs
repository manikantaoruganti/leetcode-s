public class Solution {
    public bool IsIsomorphic(string s, string t) {
        if (s.Length != t.Length) return false;

        var mapST = new Dictionary<char, char>();
        var mapTS = new Dictionary<char, char>();

        for (int i = 0; i < s.Length; i++) {
            char c1 = s[i];
            char c2 = t[i];

            if (mapST.ContainsKey(c1)) {
                if (mapST[c1] != c2) return false;
            } else {
                mapST[c1] = c2;
            }

            if (mapTS.ContainsKey(c2)) {
                if (mapTS[c2] != c1) return false;
            } else {
                mapTS[c2] = c1;
            }
        }

        return true;
    }
}
