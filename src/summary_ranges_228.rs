/*
* @lc app=leetcode id=228 lang=rust
*
* [228] Summary Ranges
*
* https://leetcode.com/problems/summary-ranges/description/
*
* algorithms
* Easy (50.95%)
* Likes:    3912
* Dislikes: 2151
* Total Accepted:    577.7K
* Total Submissions: 1.1M
* Testcase Example:  '[0,1,2,4,5,7]'
*
* You are given a sorted unique integer array nums.
*
* A range [a,b] is the set of all integers from a to b (inclusive).
*
* Return the smallest sorted list of ranges that cover all the numbers in the
* array exactly. That is, each element of nums is covered by exactly one of
* the ranges, and there is no integer x such that x is in one of the ranges
* but not in nums.
*
* Each range [a,b] in the list should be output as:
*
*
* "a->b" if a != b
* "a" if a == b
*
*
*
* Example 1:
*
*
* Input: nums = [0,1,2,4,5,7]
* Output: ["0->2","4->5","7"]
* Explanation: The ranges are:
* [0,2] --> "0->2"
* [4,5] --> "4->5"
* [7,7] --> "7"
*
*
* Example 2:
*
*
* Input: nums = [0,2,3,4,6,8,9]
* Output: ["0","2->4","6","8->9"]
* Explanation: The ranges are:
* [0,0] --> "0"
* [2,4] --> "2->4"
* [6,6] --> "6"
* [8,9] --> "8->9"
*
*
*
* Constraints:
*
*
* 0 <= nums.length <= 20
* -2^31 <= nums[i] <= 2^31 - 1
* All the values of nums are unique.
* nums is sorted in ascending order.
*
*
*  let mut res = vec![];
       let mut i = 0;
       while i < nums.len() {
           let mut j = i + 1;
           while j < nums.len() && nums[j] == nums[j - 1] + 1 {
               j += 1;
           }
           if j - i == 1 {
               res.push(nums[i].to_string());
           } else {
               res.push(format!("{}->{}", nums[i], nums[j - 1]));
           }
           i = j;
       }
       res
*
*/

// @lc code=start
#[allow(dead_code)]
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut res = vec![];
        let mut i = 0;
        while i < nums.len() {
            let mut j = i + 1;
            while j < nums.len() && nums[j] == nums[j - 1] + 1 {
                j += 1;
            }
            if j - i == 1 {
                res.push(nums[i].to_string());
            } else {
                res.push(format!("{}->{}", nums[i], nums[j - 1]));
            }
            i = j;
        }
        res
    }
}
// @lc code=end
use crate::Solution;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_228() {
        assert!(Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]) == vec!["0->2", "4->5", "7"]);
        assert!(
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]) == vec!["0", "2->4", "6", "8->9"]
        );
    }
}
