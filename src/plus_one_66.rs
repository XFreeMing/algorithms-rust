/*
 * @lc app=leetcode id=66 lang=rust
 *
 * [66] Plus One
 *
 * https://leetcode.com/problems/plus-one/description/
 *
 * algorithms
 * Easy (45.55%)
 * Likes:    9269
 * Dislikes: 5384
 * Total Accepted:    2.3M
 * Total Submissions: 5.1M
 * Testcase Example:  '[1,2,3]'
 *
 * You are given a large integer represented as an integer array digits, where
 * each digits[i] is the i^th digit of the integer. The digits are ordered from
 * most significant to least significant in left-to-right order. The large
 * integer does not contain any leading 0's.
 *
 * Increment the large integer by one and return the resulting array of
 * digits.
 *
 *
 * Example 1:
 *
 *
 * Input: digits = [1,2,3]
 * Output: [1,2,4]
 * Explanation: The array represents the integer 123.
 * Incrementing by one gives 123 + 1 = 124.
 * Thus, the result should be [1,2,4].
 *
 *
 * Example 2:
 *
 *
 * Input: digits = [4,3,2,1]
 * Output: [4,3,2,2]
 * Explanation: The array represents the integer 4321.
 * Incrementing by one gives 4321 + 1 = 4322.
 * Thus, the result should be [4,3,2,2].
 *
 *
 * Example 3:
 *
 *
 * Input: digits = [9]
 * Output: [1,0]
 * Explanation: The array represents the integer 9.
 * Incrementing by one gives 9 + 1 = 10.
 * Thus, the result should be [1,0].
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= digits.length <= 100
 * 0 <= digits[i] <= 9
 * digits does not contain any leading 0's.
 *
 *
 */

// @lc code=start
#[allow(dead_code)]
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        let mut carry = 1;
        let i = digits.len() - 1;
        if digits[i] < 9 {
            digits[i] += 1;
            return digits;
        }
        for i in (0..digits.len()).rev() {
            let sum = digits[i] + carry;
            if sum < 10 {
                digits[i] = sum;
                return digits;
            }
            digits[i] = sum % 10;
            carry = sum / 10;
        }
        if carry > 0 {
            digits.insert(0, carry);
        }
        digits
    }
}
// @lc code=end
use crate::Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plus_one() {
        assert_eq!(Solution::plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
        assert_eq!(Solution::plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
        assert_eq!(Solution::plus_one(vec![9]), vec![1, 0]);
    }
}
