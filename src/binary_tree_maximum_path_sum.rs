//! Solution for https://leetcode.com/problems/binary-tree-maximum-path-sum
//! 124. Binary Tree Maximum Path Sum

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: TreeNodePtr,
    pub right: TreeNodePtr,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

type TreeNodePtr = Option<Rc<RefCell<TreeNode>>>;

use std::cell::RefCell;
use std::rc::Rc;

fn dfs(v: &TreeNodePtr, ans: &mut i32) -> i32 {
    let mut mx1 = 0;
    let mut mx2 = 0;

    let left = &v.as_ref().unwrap().borrow().left;
    if !left.is_none() {
        let left = dfs(&left, ans);
        if left > mx1 {
            mx1 = left;
        }
    }

    let right = &v.as_ref().unwrap().borrow().right;
    if !right.is_none() {
        let right = dfs(&right, ans);
        if right > mx1 {
            mx2 = mx1;
            mx1 = right;
        } else if right > mx2 {
            mx2 = right;
        }
    }

    let val = v.as_ref().unwrap().borrow().val;
    *ans = (*ans).max(val + mx1 + mx2);
    val + mx1
}

impl Solution {
    pub fn max_path_sum(root: TreeNodePtr) -> i32 {
        let mut ans = i32::MIN;
        dfs(&root, &mut ans);
        ans
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;
// use cargo_leet::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;
    use cargo_leet::TreeRoot;

    use rstest::rstest;

    #[rstest]
    #[case(TreeRoot::from("[1,2,3]").into(), 6)]
    #[case(TreeRoot::from("[-10,9,20,null,null,15,7]").into(), 42)]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: i32) {
        let actual = Solution::max_path_sum(root);
        assert_eq!(actual, expected);
    }
}
