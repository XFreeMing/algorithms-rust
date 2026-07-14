/*
 * @lc app=leetcode id=283 lang=rust
 *
 * [283] Move Zeroes
 *
 * https://leetcode.com/problems/move-zeroes/description/
 *
 * algorithms
 * Easy (64.07%)
 * Likes:    19670
 * Dislikes: 612
 * Total Accepted:    5.4M
 * Total Submissions: 8.4M
 * Testcase Example:  '[0,1,0,3,12]'
 *
 * Given an integer array nums, move all 0's to the end of it while maintaining
 * the relative order of the non-zero elements.
 *
 * Note that you must do this in-place without making a copy of the array.
 *
 *
 * Example 1:
 * Input: nums = [0,1,0,3,12]
 * [1,0,0,3,12]
 * Output: [1,3,12,0,0]
 * Example 2:
 * Input: nums = [0]
 * Output: [0]
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 10^4
 * -2^31 <= nums[i] <= 2^31 - 1
 *
 *
 *
 * Follow up: Could you minimize the total number of operations done?
 */

// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut left = 0;
        for right in 0..nums.len() {
            if nums[right] != 0 {
                nums.swap(left, right);
                left += 1;
            }
        }
    }
}
// @lc code=end

use crate::Solution;
#[cfg(test)]
mod tests {
    #[test]
    fn example_1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        super::Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }
    #[test]
    fn example_2() {
        let mut nums = vec![0];
        super::Solution::move_zeroes(&mut nums);
        assert_eq!(nums, vec![0]);
    }
}
