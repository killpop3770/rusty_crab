pub struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut s_char_iter = s
            .chars()
            .filter(|a| a.is_alphanumeric())
            .map(|a| a.to_ascii_lowercase());

        loop {
            match (s_char_iter.next(), s_char_iter.next_back()) {
                (Some(beg_value), Some(end_value)) if beg_value == end_value => continue,
                (Some(_), Some(_)) => return false,
                _ => return true,
            }
        }
    }
}
