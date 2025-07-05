//! Solution for https://leetcode.com/problems/binary-tree-cameras
//! 968. Binary Tree Cameras

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

type NodePtr = Rc<RefCell<TreeNode>>;

// 2x2 dp[parent_has_camera][i_have_camera]
fn dfs(v: &NodePtr) -> Vec<Vec<usize>> {
    let mut children_dp = Vec::new();
    if v.borrow().left.is_some() {
        let left_dp = dfs(&v.borrow().left.as_ref().unwrap());
        children_dp.push(left_dp);
    }
    if v.borrow().right.is_some() {
        let right_dp = dfs(&v.borrow().right.as_ref().unwrap());
        children_dp.push(right_dp);
    }

    let mut dp = vec![vec![usize::MAX; 2]; 2];
    // i_have_camera == 1
    let mut dp_have_camera = 1;
    for child_dp in &children_dp {
        let cur = child_dp[1][0].min(child_dp[1][1]);
        dp_have_camera += cur;
    }
    dp[0][1] = dp_have_camera;
    dp[1][1] = dp_have_camera;

    // i_have_camera == 0
    // parent_has_camera == 0 --> at least one child should have camera
    {
        let mut sum_mins = 0;
        let mut min_diff = usize::MAX;
        for child_dp in &children_dp {
            let min = child_dp[0][0].min(child_dp[0][1]);
            sum_mins += min;
            let cur = child_dp[0][1];
            assert!(cur >= min);
            min_diff = min_diff.min(cur - min);
        }
        dp[0][0] = sum_mins + min_diff;
    }
    // parent_has_camera == 1
    {
        let mut sum = 0;
        for child_dp in &children_dp {
            let min = child_dp[0][0].min(child_dp[0][1]);
            sum += min;
        }
        dp[1][0] = sum;
    }

    dp
}

impl Solution {
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let dp = dfs(&root.unwrap());
        let ans = dp[0][0].min(dp[0][1]);
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
    #[case(TreeRoot::from("[0,0,null,0,0]").into(), 1)]
    #[case(TreeRoot::from("[0,0,null,0,null,0,null,null,0]").into(), 2)]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: i32) {
        let actual = Solution::min_camera_cover(root);
        assert_eq!(actual, expected);
    }
}
