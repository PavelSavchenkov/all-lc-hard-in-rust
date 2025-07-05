//! Solution for https://leetcode.com/problems/smallest-missing-genetic-value-in-each-subtree
//! 2003. Smallest Missing Genetic Value in Each Subtree

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

struct Set {
    s: Rc<RefCell<HashSet<i32>>>,
    mex: i32,
}

impl Set {
    fn new() -> Self {
        Self {
            s: Rc::new(RefCell::new(HashSet::new())),
            mex: 1,
        }
    }

    fn add(&mut self, num: i32) {
        assert!(num >= 1);
        self.s.borrow_mut().insert(num);
        let s = self.s.borrow();
        while s.contains(&self.mex) {
            self.mex += 1
        }
    }

    fn merge(self, other: Self) -> Self {
        if self.s.borrow().len() < other.s.borrow().len() {
            return other.merge(self);
        }

        let mut res = self;
        for &x in other.s.borrow().iter() {
            res.add(x);
        }
        res
    }
}

fn dfs(v: usize, children: &Vec<Vec<usize>>, nums: &Vec<i32>, ans: &mut Vec<i32>) -> Set {
    let mut s = Set::new();
    s.add(nums[v]);

    for &to in &children[v] {
        let s_to = dfs(to, children, nums, ans);
        s = s.merge(s_to);
    }

    ans[v] = s.mex;
    s
}

impl Solution {
    pub fn smallest_missing_value_subtree(parents: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let n = parents.len();
        let mut children = vec![Vec::<usize>::new(); n];
        for i in 1..n {
            let p = parents[i];
            assert!(p >= 0);
            children[p as usize].push(i);
        }

        let mut ans = vec![0; n];
        dfs(0, &children, &nums, &mut ans);

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
    #[case(vec![-1,0,0,2], vec![1,2,3,4], vec![5,1,1,1])]
    #[case(vec![-1,0,1,0,3,3], vec![5,4,6,2,1,3], vec![7,1,1,4,2,1])]
    #[case(vec![-1,2,3,0,2,4,1], vec![2,3,4,5,6,7,8], vec![1,1,1,1,1,1,1])]
    fn case(#[case] parents: Vec<i32>, #[case] nums: Vec<i32>, #[case] expected: Vec<i32>) {
        let actual = Solution::smallest_missing_value_subtree(parents, nums);
        assert_eq!(actual, expected);
    }
}
