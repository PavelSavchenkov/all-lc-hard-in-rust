//! Solution for https://leetcode.com/problems/find-x-sum-of-all-k-long-subarrays-ii
//! 3321. Find X-Sum of All K-Long Subarrays II

use std::collections::{BTreeMap, BTreeSet};

#[derive(Default)]
struct Set {
    top: BTreeSet<(usize, i32)>,
    cnt_top: usize,
    sum_top: i64,
    bottom: BTreeSet<(usize, i32)>,
    cnt_of: BTreeMap<i32, usize>,
}

impl Set {
    fn new(cnt_top: usize) -> Self {
        Self {
            cnt_top,
            ..Default::default()
        }
    }

    fn remove_top(&mut self, item: (usize, i32)) {
        if self.top.remove(&item) {
            self.sum_top -= item.0 as i64 * item.1 as i64;
        }
    }

    fn insert_top(&mut self, item: (usize, i32)) {
        if self.top.insert(item) {
            self.sum_top += item.0 as i64 * item.1 as i64;
        }
    }

    fn insert(&mut self, item: (usize, i32)) {
        if !self.bottom.is_empty() && item < *self.bottom.last().unwrap() {
            self.bottom.insert(item);
        } else {
            self.insert_top(item);
        }
        self.norm();
    }

    fn remove(&mut self, item: (usize, i32)) {
        self.remove_top(item);
        self.bottom.remove(&item);
        self.norm();
    }

    fn insert_one(&mut self, x: i32) {
        let cnt = self.cnt_of.entry(x).or_insert(0);
        *cnt += 1;
        let new_cnt = *cnt;
        self.remove((new_cnt - 1, x));
        self.insert((new_cnt, x));
    }

    fn remove_one(&mut self, x: i32) {
        let cnt = self.cnt_of.entry(x).or_insert(0);
        *cnt -= 1;
        let new_cnt = *cnt;
        self.remove((new_cnt + 1, x));
        self.insert((new_cnt, x));
    }

    fn norm(&mut self) {
        // each in the top > each in the bottom
        while self.top.len() < self.cnt_top && !self.bottom.is_empty() {
            let item = self.bottom.pop_last().unwrap();
            self.insert_top(item);
        }
        while self.top.len() > self.cnt_top {
            let item = *self.top.first().unwrap();
            self.remove_top(item);
            self.bottom.insert(item);
        }
    }
}

impl Solution {
    pub fn find_x_sum(a: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        let n = a.len();
        let x = x as usize;
        let k = k as usize;

        let mut ans = Vec::new();
        let mut set = Set::new(x);
        let mut l = 0;
        let mut r = 0;
        for i in 0..=n - k {
            while l < i {
                set.remove_one(a[l]);
                l += 1;
            }
            while r < i + k {
                set.insert_one(a[r]);
                r += 1;
            }
            let cur = set.sum_top;
            ans.push(cur);
        }

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
    #[case(vec![1,1,2,2,3,4,2,3], 6, 2, vec![6,10,12])]
    #[case(vec![3,8,7,8,7,5], 2, 2, vec![11,15,15,15,12])]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] x: i32, #[case] expected: Vec<i64>) {
        let actual = Solution::find_x_sum(nums, k, x);
        assert_eq!(actual, expected);
    }
}
