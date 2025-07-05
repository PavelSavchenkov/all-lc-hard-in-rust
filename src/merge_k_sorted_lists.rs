//! Solution for https://leetcode.com/problems/merge-k-sorted-lists
//! 23. Merge k Sorted Lists

use std::collections::BinaryHeap;

type ListNodePtr = Option<Box<ListNode>>;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: ListNodePtr,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from_list(vals: &[i32]) -> ListNodePtr {
        let mut prev = None;
        for &val in vals.iter().rev() {
            let mut cur = Self::new(val);
            cur.next = prev;
            prev = Some(Box::new(cur));
        }
        prev
    }

    fn to_list(mut ptr: ListNodePtr) -> Vec<i32> {
        let mut vals = Vec::new();
        while !ptr.is_none() {
            vals.push(ptr.as_ref().unwrap().val);
            ptr = ptr.unwrap().next;
        }
        vals
    }
}

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        for (i, head) in lists.iter().enumerate() {
            if head.is_none() {
                continue;
            }
            let val = head.as_ref().unwrap().val;
            heap.push((-val, i));
        }

        let mut ans = Vec::new();
        while !heap.is_empty() {
            let (mut val, i) = heap.pop().unwrap();
            val = -val;

            ans.push(val);

            let head = &lists[i];
            lists[i] = head.as_ref().unwrap().next.clone();
            if lists[i].is_none() {
                continue;
            }
            let new_val = lists[i].as_ref().unwrap().val;
            heap.push((-new_val, i));
        }

        ListNode::from_list(&ans)
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![1,4,5],vec![1,3,4],vec![2,6]], vec![1,1,2,3,4,4,5,6])]
    #[case(vec![], vec![])]
    #[case(vec![vec![]], vec![])]
    fn case(#[case] lists: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let lists: Vec<_> = lists.iter().map(|list| ListNode::from_list(list)).collect();
        let actual = Solution::merge_k_lists(lists);
        let actual = ListNode::to_list(actual);
        assert_eq!(actual, expected);
    }
}
