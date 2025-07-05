//! Solution for https://leetcode.com/problems/recover-a-tree-from-preorder-traversal
//! 1028. Recover a Tree From Preorder Traversal

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

struct Token {
    depth: usize,
    val: i32,
}

type TreePtr = Rc<RefCell<TreeNode>>;

fn dfs(tokens: &Vec<Token>, pos: &mut usize, depth: usize) -> Option<TreePtr> {
    assert!(*pos < tokens.len());
    assert!(tokens[*pos].depth == depth);
    let mut t = TreeNode::new(tokens[*pos].val);
    *pos += 1;

    if *pos < tokens.len() && tokens[*pos].depth == depth + 1 {
        t.left = dfs(tokens, pos, depth + 1);
    }
    if *pos < tokens.len() && tokens[*pos].depth == depth + 1 {
        t.right = dfs(tokens, pos, depth + 1);
    }

    Some(Rc::new(RefCell::new(t)))
}

impl Solution {
    pub fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
        let s = to_u8(&traversal);

        let mut tokens = Vec::new();
        let mut i = 0;
        while i < s.len() {
            let mut depth = 0;
            while s[i] == b'-' {
                depth += 1;
                i += 1;
            }
            let mut val = 0;
            while i < s.len() && s[i].is_ascii_digit() {
                val = val * 10 + (s[i] - b'0') as i32;
                i += 1;
            }
            tokens.push(Token { depth, val });
        }
        let mut pos = 0;
        let ans = dfs(&tokens, &mut pos, 0);
        assert!(pos == tokens.len());
        ans
    }
}

fn to_u8(s: &String) -> Vec<u8> {
    s.as_bytes().iter().map(|&c| c as u8).collect()
}

fn to_u8_vec(s: &Vec<String>) -> Vec<Vec<u8>> {
    s.iter().map(|ss| to_u8(&ss)).collect()
}

fn from_u8(s: &Vec<u8>) -> String {
    String::from_utf8(s.to_vec()).unwrap()
}

fn from_u8_vec(s: &Vec<Vec<u8>>) -> Vec<String> {
    s.iter().map(|ss| from_u8(&ss)).collect()
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
    #[case("1-2--3--4-5--6--7", TreeRoot::from("[1,2,5,3,4,6,7]").into())]
    #[case("1-2--3---4-5--6---7", TreeRoot::from("[1,2,5,3,null,6,null,4,null,7]").into())]
    #[case("1-401--349---90--88", TreeRoot::from("[1,401,null,349,88,90]").into())]
    fn case(#[case] traversal: String, #[case] expected: Option<Rc<RefCell<TreeNode>>>) {
        let actual = Solution::recover_from_preorder(traversal);
        assert_eq!(actual, expected);
    }
}
