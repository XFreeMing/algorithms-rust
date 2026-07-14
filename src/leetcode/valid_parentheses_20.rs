/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 *
 * https://leetcode.com/problems/valid-parentheses/description/
 *
 * algorithms
 * Easy (40.68%)
 * Likes:    23893
 * Dislikes: 1748
 * Total Accepted:    4.7M
 * Total Submissions: 11.6M
 * Testcase Example:  '"()"'
 *
 * Given a string s containing just the characters '(', ')', '{', '}', '[' and
 * ']', determine if the input string is valid.
 *
 * An input string is valid if:
 *
 *
 * Open brackets must be closed by the same type of brackets.
 * Open brackets must be closed in the correct order.
 * Every close bracket has a corresponding open bracket of the same type.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: s = "()"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: s = "()[]{}"
 * Output: true
 *
 *
 * Example 3:
 *
 *
 * Input: s = "(]"
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 10^4
 * s consists of parentheses only '()[]{}'.
 *
 *
 */

/**
 *
 * Thoughts:
 * use stack to solve this problem
 * if open bracket push to stack
 * if close bracket pop a element from stack,
 * if the element is not match the open bracket, return false
 *
 */
// @lc code=start
#[allow(dead_code)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                _ => return false,
            }
        }
        stack.is_empty()
    }
}
// @lc code=end

use crate::Solution;
#[cfg(test)]
mod tests {
    use super::*;
    /**
     * Open brackets must be closed by the same type of brackets.
     */
    #[test]
    fn test_1() {
        // good case
        let s = "()".to_string();
        let result = Solution::is_valid(s);
        assert!(result);
        // bad case
        let s = "(".to_string();
        let result = Solution::is_valid(s);
        assert!(!result);
        // bad case
        let s = ")".to_string();
        let result = Solution::is_valid(s);
        assert!(!result);
    }
    // Open brackets must be closed in the correct order.
    #[test]
    fn test_2() {
        let s = "()[]{}".to_string();
        let result: bool = Solution::is_valid(s);
        assert!(result);
    }
    // Every close bracket has a corresponding open bracket of the same type.
    #[test]
    fn test_3() {
        let s = "(]".to_string();
        let result = Solution::is_valid(s);
        assert!(!result);
    }
}
