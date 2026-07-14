/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 *
 * https://leetcode.com/problems/container-with-most-water/description/
 *
 * algorithms
 * Medium (60.37%)
 * Likes:    34614
 * Dislikes: 2224
 * Total Accepted:    5.6M
 * Total Submissions: 9.3M
 * Testcase Example:  '[1,8,6,2,5,4,8,3,7]'
 *
 * You are given an integer array height of length n. There are n vertical
 * lines drawn such that the two endpoints of the i^th line are (i, 0) and (i,
 * height[i]).
 * >
 * Find two lines that together with the x-axis form a container, such that the
 * container contains the most water.
 *
 * Return the maximum amount of water a container can store.
 *
 * Notice that you may not slant the container.
 *
 *
 * Example 1:
 *
 *
 * Input: height = [1,8,6,2,5,4,8,3,7]
 * Output: 49
 * Explanation: The above vertical lines are represented by array
 * [1,8,6,2,5,4,8,3,7]. In this case, the max area of water (blue section) the
 * container can contain is 49.
 *
 *
 * Example 2:
 *
 *
 * Input: height = [1,1]
 * Output: 1
 *
 *
 *
 * Constraints:
 *
 *
 * n == height.length
 * 2 <= n <= 10^5
 * 0 <= height[i] <= 10^4
 *
 *
 */
// @lc code=start
#[allow(dead_code)]
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut result = 0;
        while i < j {
            let area = (j - i) as i32 * std::cmp::min(height[i], height[j]);
            result = std::cmp::max(result, area);
            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }
        result
    }
}
// @lc code=end
use crate::Solution;
#[cfg(test)]
mod tests {
    #[test]
    fn example_1() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        let result = super::Solution::max_area(height);
        assert_eq!(result, 49);
    }
    #[test]
    fn example_2() {
        let height = vec![1, 1];
        let result = super::Solution::max_area(height);
        assert_eq!(result, 1);
    }
}
