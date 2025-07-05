//! Solution for https://leetcode.com/problems/merge-bsts-to-create-single-bst
//! 1932. Merge BSTs to Create Single BST

// Definition for a binary tree node.

use core::panic;
use std::cell::RefMut;

type TreeNodePtr = Option<Rc<RefCell<TreeNode>>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: TreeNodePtr,
    pub right: TreeNodePtr,
}

fn print_tree(t: &TreeNodePtr) {
    if t.is_none() {
        return;
    }
    let tt = unwrap_tree_node_mut(&t);
    eprintln!("value = {}", tt.val);

    if !tt.left.is_none() {
        eprintln!("going left from {} ...", tt.val);
        print_tree(&tt.left);
    }

    if !tt.right.is_none() {
        eprintln!("going right from {} ...", tt.val);
        print_tree(&tt.right);
    }

    eprintln!("coming out of value = {}", tt.val);
}

fn check_if_bst(t: &TreeNodePtr, min: i32, max: i32) -> bool {
    if t.is_none() {
        return true;
    }
    let tt = unwrap_tree_node_mut(&t);
    let val = tt.val;
    if val < min || val > max {
        return false;
    }

    if !check_if_bst(&tt.left, min, max.min(val - 1)) {
        return false;
    }
    if !check_if_bst(&tt.right, min.max(val + 1), max) {
        return false;
    }
    true
}

fn unwrap_tree_node_mut(t: &TreeNodePtr) -> RefMut<'_, TreeNode> {
    t.as_ref().unwrap().borrow_mut()
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

    pub fn new_ptr(val: i32) -> TreeNodePtr {
        Some(Rc::new(RefCell::new(TreeNode::new(val))))
    }

    // O(n^2)
    pub fn from_list(vals: &[i32]) -> TreeNodePtr {
        // dep 0, then dep 1, then dep 2, etc
        if vals.is_empty() {
            return None;
        }
        if vals[0] == -1 {
            assert!(vals.len() == 1);
            return None;
        }
        let root = Self::new_ptr(vals[0]);
        let to_left: Vec<_> = vals.iter().copied().filter(|&v| v < vals[0]).collect();
        let to_right: Vec<_> = vals.iter().copied().filter(|&v| v > vals[0]).collect();
        assert!(to_left.len() + to_right.len() + 1 == vals.len());
        unwrap_tree_node_mut(&root).left = Self::from_list(&to_left);
        unwrap_tree_node_mut(&root).right = Self::from_list(&to_right);
        root
    }

    pub fn check_if_bst_same(t1: TreeNodePtr, t2: TreeNodePtr) -> bool {
        if t1.is_none() {
            if t2.is_none() {
                return true;
            }
            eprintln!("Err: t1 is none, but t2 is not none");
            return false;
        }
        if t2.is_none() {
            eprintln!("Err: t1 is not none, but t2 is none");
            return false;
        }
        if unwrap_tree_node_mut(&t1).val != unwrap_tree_node_mut(&t2).val {
            eprintln!("Err: vals not equal");
            return false;
        }
        return true;
    }
}

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

fn dfs(v: usize, children: &Vec<Vec<usize>>, was: &mut [bool]) -> bool {
    if was[v] {
        return false;
    }
    was[v] = true;

    for &child in &children[v] {
        if !dfs(child, children, was) {
            return false;
        }
    }

    true
}

impl Solution {
    pub fn can_merge(trees: Vec<TreeNodePtr>) -> TreeNodePtr {
        let n = trees.len();
        let mut leaf_with = HashMap::<i32, usize>::new();
        for (i, t) in trees.iter().enumerate() {
            let tt = unwrap_tree_node_mut(&t);
            for child_ptr in [&tt.left, &tt.right] {
                if child_ptr.is_none() {
                    continue;
                }
                let child = unwrap_tree_node_mut(&child_ptr);
                if leaf_with.contains_key(&child.val) {
                    return None;
                }
                leaf_with.insert(child.val, i);
            }
        }

        let mut par = vec![-1 as i32; n];
        let mut children = vec![Vec::<usize>::new(); n];
        let mut root: i32 = -1;
        for (i, t) in trees.iter().enumerate() {
            let val = unwrap_tree_node_mut(&t).val;
            match leaf_with.get(&val) {
                Some(idx) => {
                    let p = *idx;
                    par[i] = p as i32;
                    children[p].push(i);
                }
                None => {
                    if root != -1 {
                        return None;
                    }
                    root = i as i32;
                }
            }
        }

        if root == -1 {
            return None;
        }

        // check for potential cycles
        let mut was = vec![false; n];
        if !dfs(root as usize, &children, &mut was) {
            return None;
        }
        if !was.iter().all(|&w| w) {
            return None;
        }

        for i in 0..n {
            let p = par[i];
            if p == -1 {
                continue;
            }
            let t = unwrap_tree_node_mut(&trees[i]);
            let mut par_t = unwrap_tree_node_mut(&trees[p as usize]);
            let try_attach = |ptr: &mut TreeNodePtr| -> bool {
                if ptr.is_none() {
                    return false;
                }
                {
                    let node = unwrap_tree_node_mut(&ptr);
                    if node.val != t.val {
                        return false;
                    }
                }
                *ptr = Some(trees[i].as_ref().unwrap().clone());
                true
            };
            let found = try_attach(&mut par_t.left) || try_attach(&mut par_t.right);
            assert!(found);
        }

        let ans = trees[root as usize].clone();

        if !check_if_bst(&ans, i32::MIN, i32::MAX) {
            return None;
        }

        // print_tree(&ans);

        ans
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![2,1],vec![3,2,5],vec![5,4]], vec![3,2,5,1,-1,4])]
    #[case(vec![vec![5, 3, 8],vec![3,2,6]], vec![-1])]
    #[case(vec![vec![5, 4],vec![3]], vec![-1])]
    fn case(#[case] trees: Vec<Vec<i32>>, #[case] expected: Vec<i32>) {
        let trees = trees.iter().map(|v| TreeNode::from_list(&v)).collect();
        let expected = TreeNode::from_list(&expected);
        let actual = Solution::can_merge(trees);
        assert!(TreeNode::check_if_bst_same(actual, expected));
    }
}
