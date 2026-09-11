impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: String = s
            .chars()
            .filter(|c| c.is_alphabetic() || c.is_numeric())
            .collect();

        let s = s.to_lowercase();
        let s_reverse: String = s.chars().rev().collect();

        s == s_reverse
    }
}
