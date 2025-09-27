/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 *
 * https://leetcode.com/problems/add-two-numbers/description/
 * algorithms
 * Medium (46.24%)
 * Likes:    34854
 * Dislikes: 6973
 * Total Accepted:    6.2M
 * Total Submissions: 13.3M
 * Testcase Example:  '[2,4,3]\n[5,6,4]'
 *
 * You are given two non-empty linked lists representing two non-negative integers.
 * - non-empty 非空
 * - linked list 链表
 * - representing 代表
 * - non-negative integers 非负整数
 * The digits are stored in reverse order, and each of their nodes
 * contains a single digit. Add the two numbers and return the sum as a linked
 * list.
 * - digits 数字
 * - stored 存储
 * - reverse order 逆序
 *
 * You may assume the two numbers do not contain any leading zero, except the
 * number 0 itself.
 * - assume 假设
 * - leading zero 前导零
 * - except 除了
 *
 * Example 1:
 *
 *
 * Input: l1 = [2,4,3], l2 = [5,6,4]
 * Output: [7,0,8]
 * Explanation: 342 + 465 = 807.
 *
 *
 * Example 2:
 *
 *
 * Input: l1 = [0], l2 = [0]
 * Output: [0]
 *
 *
 * Example 3:
 *
 *
 * Input: l1 = [9,9,9,9,9,9,9], l2 = [9,9,9,9]
 * Output: [8,9,9,9,0,0,0,1]
 *
 *
 *
 * Constraints: 约束
 *
 *
 * The number of nodes in each linked list is in the range [1, 100].
 * 0 <= Node.val <= 9
 * It is guaranteed that the list represents a number that does not have
 * leading zeros.
 *
 *
 *
 */
#[derive(PartialEq, Eq, Clone, Debug)]
#[allow(dead_code)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}
#[allow(dead_code)]
impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
// @lc code=start
// Definition for singly-linked list
#[allow(dead_code)]
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let (mut p, mut q) = (l1, l2);
        let mut carry = 0;
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        while p.is_some() || q.is_some() || carry != 0 {
            let (x, next_p) = match p {
                Some(node) => (node.val, node.next),
                None => (0, None),
            };
            p = next_p;
            let (y, next_q) = match q {
                Some(node) => (node.val, node.next),
                None => (0, None),
            };
            q = next_q;
            let sum = x + y + carry;
            carry = sum / 10;
            tail.next = Some(Box::new(ListNode::new(sum % 10)));
            tail = tail.next.as_mut().unwrap();
        }
        dummy.next
    }
}
// @lc code=end

use crate::Solution;

#[cfg(test)]
mod tests {
    use super::*;

    fn to_list(nums: &[i32]) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        for &v in nums.iter().rev() {
            // build in reverse to keep order
            let mut node = Box::new(ListNode::new(v));
            node.next = head;
            head = Some(node);
        }
        head
    }

    fn to_vec(mut list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut v = Vec::new();
        while let Some(node) = list {
            v.push(node.val);
            list = node.next;
        }
        v
    }

    #[test]
    fn test_example1() {
        let l1 = to_list(&[2, 4, 3]);
        let l2 = to_list(&[5, 6, 4]);
        let res = Solution::add_two_numbers(l1, l2);
        assert_eq!(to_vec(res), vec![7, 0, 8]);
    }

    #[test]
    fn test_example2() {
        let l1 = to_list(&[0]);
        let l2 = to_list(&[0]);
        let res = Solution::add_two_numbers(l1, l2);
        assert_eq!(to_vec(res), vec![0]);
    }

    #[test]
    fn test_example3() {
        let l1 = to_list(&[9, 9, 9, 9, 9, 9, 9]);
        let l2 = to_list(&[9, 9, 9, 9]);
        let res = Solution::add_two_numbers(l1, l2);
        assert_eq!(to_vec(res), vec![8, 9, 9, 9, 0, 0, 0, 1]);
    }
}
