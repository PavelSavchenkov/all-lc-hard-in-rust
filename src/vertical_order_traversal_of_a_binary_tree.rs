//! Solution for https://leetcode.com/problems/vertical-order-traversal-of-a-binary-tree
//! 987. Vertical Order Traversal of a Binary Tree

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
use std::collections::BTreeMap;
use std::rc::Rc;

type TreePtr = Rc<RefCell<TreeNode>>;

fn dfs(v: &TreePtr, row: i32, col: i32, ans: &mut BTreeMap<i32, Vec<(i32, i32)>>) {
    ans.entry(col)
        .or_insert(Vec::new())
        .push((row, v.borrow().val));

    let v_node = v.borrow();
    if v_node.left.is_some() {
        dfs(&v_node.left.as_ref().unwrap(), row + 1, col - 1, ans);
    }
    if v_node.right.is_some() {
        dfs(&v_node.right.as_ref().unwrap(), row + 1, col + 1, ans);
    }
}

impl Solution {
    pub fn vertical_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = BTreeMap::<i32, Vec<(i32, i32)>>::new();
        dfs(&root.unwrap(), 0, 0, &mut ans);
        let mut ret = Vec::new();
        for (col, vec) in &mut ans {
            vec.sort();
            let vec: Vec<i32> = vec.iter().map(|(row, val)| *val).collect();
            ret.push(vec);
        }
        ret
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
    #[case(TreeRoot::from("[3,9,20,null,null,15,7]").into(), vec![vec![9],vec![3,15],vec![20],vec![7]])]
    #[case(TreeRoot::from("[1,2,3,4,5,6,7]").into(), vec![vec![4],vec![2],vec![1,5,6],vec![3],vec![7]])]
    #[case(TreeRoot::from("[1,2,3,4,6,5,7]").into(), vec![vec![4],vec![2],vec![1,5,6],vec![3],vec![7]])]
    fn case(#[case] root: Option<Rc<RefCell<TreeNode>>>, #[case] expected: Vec<Vec<i32>>) {
        let actual = Solution::vertical_traversal(root);
        assert_eq!(actual, expected);
    }
}
