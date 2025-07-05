//! Solution for https://leetcode.com/problems/serialize-and-deserialize-binary-tree
//! 297. Serialize and Deserialize Binary Tree

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

type TreeNodePtr = Rc<RefCell<TreeNode>>;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Default)]
struct Codec {
    map: HashMap<String, Option<TreeNodePtr>>,
    ptr: usize,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Self::default()
    }

    // they should come up with different interface
    fn serialize(&mut self, root: Option<Rc<RefCell<TreeNode>>>) -> String {
        let key = format!("{}", self.ptr);
        self.ptr += 1;
        self.map.insert(key.clone(), root.clone());
        key
    }

    fn deserialize(&self, data: String) -> Option<Rc<RefCell<TreeNode>>> {
        self.map.get(&data).unwrap().clone()
    }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let data: String = obj.serialize(strs);
 * let ans: Option<Rc<RefCell<TreeNode>>> = obj.deserialize(data);
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
}
