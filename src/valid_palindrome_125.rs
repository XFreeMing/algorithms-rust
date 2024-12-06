/*
 * @lc app=leetcode id=125 lang=rust
 *
 * [125] Valid Palindrome
 *
 * https://leetcode.com/problems/valid-palindrome/description/
 *
 * algorithms
 * Easy (49.29%)
 * Likes:    9779
 * Dislikes: 8459
 * Total Accepted:    3.6M
 * Total Submissions: 7.4M
 * Testcase Example:  '"A man, a plan, a canal: Panama"'
 *
 * A phrase is a palindrome if, after converting all uppercase letters into
 * lowercase letters and removing all non-alphanumeric characters, it reads the
 * same forward and backward. Alphanumeric characters include letters and
 * numbers.
 *
 * Given a string s, return true if it is a palindrome, or false otherwise.
 *
 *
 * Example 1:
 *
 *
 * Input: s = "A man, a plan, a canal: Panama"
 * Output: true
 * Explanation: "amanaplanacanalpanama" is a palindrome.
 *
 *
 * Example 2:
 *
 *
 * Input: s = "race a car"
 * Output: false
 * Explanation: "raceacar" is not a palindrome.
 *
 *
 * Example 3:
 *
 *
 * Input: s = " "
 * Output: true
 * Explanation: s is an empty string "" after removing non-alphanumeric
 * characters.
 * Since an empty string reads the same forward and backward, it is a
 * palindrome.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 2 * 10^5
 * s consists only of printable ASCII characters.
 *
 *
 */

// @lc code=start
#[allow(dead_code)]
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        // 1. filter out number & char;
        let filtered = Self::filter_non_number_and_char(s);

        // 2. reverse and compare
        let reversed = Self::reverse_string(filtered.clone());
        filtered == reversed
    }
    fn filter_non_number_and_char(s: String) -> String {
        let mut filtered = String::new();
        for c in s.chars() {
            if c.is_ascii_alphanumeric() {
                filtered.push(c.to_ascii_lowercase());
            }
        }
        filtered
    }
    fn reverse_string(s: String) -> String {
        s.chars().rev().collect::<String>()
    }
}
// @lc code=end
use crate::Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_non_number_and_char() {
        assert_eq!(
            Solution::filter_non_number_and_char("A man, a plan, a canal: Panama".to_string()),
            "amanaplanacanalpanama"
        );
        assert_eq!(
            Solution::filter_non_number_and_char("race a car".to_string()),
            "raceacar"
        );
        assert_eq!(Solution::filter_non_number_and_char(" ".to_string()), "");
    }

    #[test]
    fn test_reverse_string() {
        assert_eq!(
            Solution::reverse_string("amanaplanacanalpanama".to_string()),
            "amanaplanacanalpanama"
        );
        assert_eq!(Solution::reverse_string("".to_string()), "");
        assert_eq!(Solution::reverse_string("raceacar".to_string()), "racaecar");
    }

    #[test]
    fn test_is_palindrome() {
        assert!(Solution::is_palindrome(
            "A man, a plan, a canal: Panama".to_string()
        ));
        assert!(!Solution::is_palindrome("race a car".to_string()));
        assert!(Solution::is_palindrome(" ".to_string()));
    }
}
