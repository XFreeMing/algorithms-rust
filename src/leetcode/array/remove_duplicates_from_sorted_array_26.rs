/*
 * @lc app=leetcode id=26 lang=rust
 *
 * [26] Remove Duplicates from Sorted Array
 * > 删除排序数组中的重复项
 * https://leetcode.com/problems/remove-duplicates-from-sorted-array/description/
 *
 * algorithms
 * Easy (58.38%)
 * Likes:    15570
 * Dislikes: 19232
 * Total Accepted:    5.3M
 * Total Submissions: 9.1M
 * Testcase Example:  '[1,1,2]'
 *
 * Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once.
 * > 给一个非递减排序的整数数组，删除重复的元素，使得每个唯一的元素只出现一次。
 * The relative order of the elements should be kept the same. Then return the number of unique elements in nums.
 * > 元素的相对顺序应该保持不变。然后返回 nums 中的唯一元素的数量。
 * Consider the number of unique elements of nums to be k, to get accepted, you
 * need to do the following things:
 * > 将 nums 中的唯一元素数量记为 k，为了被接受，你需要做以下事情：
 *
 * Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially.
 *
 * The remaining elements of nums are not important as well as the size of nums.
 *
 * Return k.
 *
 *
 * Custom Judge:
 *
 * The judge will test your solution with the following code:
 * > 评判你的代码
 *
 * int[] nums = [...]; // Input array
 * int[] expectedNums = [...]; // The expected answer with correct length
 *
 * int k = removeDuplicates(nums); // Calls your implementation
 *
 * assert k == expectedNums.length;
 * for (int i = 0; i < k; i++) {
 * ⁠   assert nums[i] == expectedNums[i];
 * }
 *
 *
 * If all assertions pass, then your solution will be accepted.
 *
 *
 * Example 1:
 *
 *
 * Input: nums = [1,1,2]
 * Output: 2, nums = [1,2,_]
 * Explanation: Your function should return k = 2, with the first two elements
 * of nums being 1 and 2 respectively.
 * It does not matter what you leave beyond the returned k (hence they are
 * underscores).
 *
 *
 * Example 2:
 *
 *
 * Input: nums = [0,0,1,1,1,2,2,3,3,4]
 * Output: 5, nums = [0,1,2,3,4,_,_,_,_,_]
 * Explanation: Your function should return k = 5, with the first five elements
 * of nums being 0, 1, 2, 3, and 4 respectively.
 * It does not matter what you leave beyond the returned k (hence they are
 * underscores).
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= nums.length <= 3 * 10^4
 * -100 <= nums[i] <= 100
 * nums is sorted in non-decreasing order.
 *
 *
 */

// @lc code=start
#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        Self::solution_2(nums)
    }

    fn solution_1(nums: &mut [i32]) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut j = 0;
        for i in 1..nums.len() {
            if nums[j] != nums[i] {
                j += 1;
                nums[j] = nums[i];
            }
        }

        (j + 1) as i32
    }
    fn solution_2(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }
}
// @lc code=end
use crate::Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_duplicates_1() {
        let mut nums = vec![1, 1, 2];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, 2);
        assert_eq!(nums[..k as usize], [1, 2]);
    }

    #[test]
    fn test_remove_duplicates_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, 5);
        assert_eq!(nums[..k as usize], [0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_remove_duplicates_3() {
        let mut nums = vec![1, 1, 1, 1, 1];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, 1);
        assert_eq!(nums[..k as usize], [1]);
    }

    #[test]
    fn test_remove_duplicates_4() {
        let mut nums = vec![1, 2, 3, 4, 5];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, 5);
        assert_eq!(nums[..k as usize], [1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_remove_duplicates_5() {
        let mut nums = vec![];
        let k = Solution::remove_duplicates(&mut nums);
        assert_eq!(k, 0);
        assert_eq!(nums[..k as usize], []);
    }
}
