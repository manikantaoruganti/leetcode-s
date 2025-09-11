class Solution {
    func kthCharacter(_ k: Int) -> Character {
        var word = "a"
        
        // Helper function to get next character with wrap-around
        func nextChar(_ c: Character) -> Character {
            if c == "z" { return "a" }
            return Character(UnicodeScalar(c.asciiValue! + 1))
        }
        
        // Keep generating until length >= k
        while word.count < k {
            let transformed = word.map { nextChar($0) }
            word += String(transformed)
        }
        
        // Return the k-th character (1-based index)
        return word[word.index(word.startIndex, offsetBy: k - 1)]
    }
}
