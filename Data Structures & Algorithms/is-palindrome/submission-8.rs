impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let s: String = s
            .chars()
            .filter(|c| c.is_alphabetic() || c.is_numeric())
            .collect();

        if s.len() <= 1 {
            return true;
        }

        let s = s.to_lowercase();
        let s_reverse = s.chars().rev().collect();

        s.cmp(&s_reverse) == Ordering::Equal
    }
}
