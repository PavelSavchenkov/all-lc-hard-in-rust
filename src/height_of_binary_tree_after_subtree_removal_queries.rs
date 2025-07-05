//! Solution for https://leetcode.com/problems/height-of-binary-tree-after-subtree-removal-queries
//! 2458. Height of Binary Tree After Subtree Removal Queries

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
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

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
type TreeNodePtr = Option<Rc<RefCell<TreeNode>>>;

fn dfs_height(t: &TreeNodePtr, mem: &mut HashMap<i32, usize>) -> usize {
    let mut h = 0;

    let t_node = t.as_ref().unwrap().borrow();

    if t_node.left.is_some() {
        let h_left = dfs_height(&t_node.left, mem);
        h = h.max(h_left + 1);
    }
    if t_node.right.is_some() {
        let h_right = dfs_height(&t_node.right, mem);
        h = h.max(h_right + 1);
    }

    mem.insert(t_node.val, h);

    h
}

fn dfs(
    t: &TreeNodePtr,
    depth: i32,
    mx_outside: i32,
    qs_for_val: &HashMap<i32, Vec<usize>>,
    height: &HashMap<i32, usize>,
    answers: &mut Vec<i32>,
) {
    let t_node = t.as_ref().unwrap().borrow();

    let cur_ans = (depth - 1).max(mx_outside).max(0);
    let qs = qs_for_val.get(&t_node.val);
    if qs.is_some() {
        for &id in qs.unwrap() {
            answers[id] = cur_ans;
        }
    }

    let mut left_height: i32 = -1;
    if t_node.left.is_some() {
        left_height = *height
            .get(&t_node.left.as_ref().unwrap().borrow().val)
            .unwrap() as i32;
    }

    let mut right_height: i32 = -1;
    if t_node.right.is_some() {
        right_height = *height
            .get(&t_node.right.as_ref().unwrap().borrow().val)
            .unwrap() as i32;
    }

    if t_node.left.is_some() {
        dfs(
            &t_node.left,
            depth + 1,
            mx_outside.max(depth + 1 + right_height),
            qs_for_val,
            height,
            answers,
        );
    }
    if t_node.right.is_some() {
        dfs(
            &t_node.right,
            depth + 1,
            mx_outside.max(depth + 1 + left_height),
            qs_for_val,
            height,
            answers,
        );
    }
}

impl Solution {
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        let m = queries.len();

        let mut answers = vec![0; m];
        let mut qs_for_val = HashMap::<i32, Vec<usize>>::new();
        for i in 0..m {
            let val = queries[i];
            qs_for_val.entry(val).or_insert(Vec::new()).push(i);
        }

        let mut height = HashMap::<i32, usize>::new();
        dfs_height(&root, &mut height);

        dfs(&root, 0, 0, &qs_for_val, &height, &mut answers);

        answers
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    // #[case(TreeRoot::from("[1,3,4,2,null,6,5,null,null,null,null,null,7]").into(), vec![4], vec![2])]
    // #[case(TreeRoot::from("[5,8,9,2,1,3,7,4,6]").into(), vec![3,2,4,8], vec![3,2,3,2])]
    fn case(// #[case] root: Option<Rc<RefCell<TreeNode>>>,
        // #[case] queries: Vec<i32>,
        // #[case] expected: Vec<i32>,
    ) {
        // let actual = Solution::tree_queries(root, queries);
        // assert_eq!(actual, expected);
    }
}
