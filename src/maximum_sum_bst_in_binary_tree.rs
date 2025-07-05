//! Solution for https://leetcode.com/problems/maximum-sum-bst-in-binary-tree
//! 1373. Maximum Sum BST in Binary Tree

// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;

struct Info {
    min_key: i32,
    max_key: i32,
    sum: i32,
}

type NodePtr = Rc<RefCell<TreeNode>>;

fn dfs(v: &NodePtr, ans: &mut i32) -> Option<Info> {
    let mut info = Info {
        min_key: i32::MAX,
        max_key: i32::MIN,
        sum: 0,
    };
    let mut is_bst = true;
    let key = v.borrow().val;

    if let Some(left_v) = v.borrow().left.as_ref() {
        match dfs(&left_v, ans) {
            None => {
                is_bst = false;
            }
            Some(left_info) => {
                if left_info.max_key >= key {
                    is_bst = false;
                } else {
                    info = left_info;
                }
            }
        }
    }
    if let Some(right_v) = v.borrow().right.as_ref() {
        match dfs(&right_v, ans) {
            None => {
                is_bst = false;
            }
            Some(right_info) => {
                if right_info.min_key <= key {
                    is_bst = false;
                } else {
                    info.max_key = right_info.max_key;
                    info.sum += right_info.sum;
                }
            }
        }
    }

    if !is_bst {
        return None;
    }

    info.sum += key;
    info.max_key = info.max_key.max(key);
    info.min_key = info.min_key.min(key);

    *ans = (*ans).max(info.sum);

    Some(info)
}

impl Solution {
    pub fn max_sum_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut ans = 0;
        dfs(&root.unwrap(), &mut ans);
        ans as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;
use cargo_leet::TreeNode;

#[cfg(test)]
mod tests {
    use super::*;
    use cargo_leet::TreeRoot;

    use rstest::rstest;

    #[rstest]
    #[case(TreeRoot::from("[1,4,3,2,4,2,5,null,null,null,null,null,null,4,6]").into(), 20)]
    #[case(TreeRoot::from("[4,3,null,1,2]").into(), 2)]
    #[case(TreeRoot::from("[-4,-2,-5]").into(), 0)]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: i32) {
        let actual = Solution::max_sum_bst(root);
        assert_eq!(actual, expected);
    }
}
