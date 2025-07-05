//! Solution for https://leetcode.com/problems/sliding-window-median
//! 480. Sliding Window Median

use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};

#[derive(Default)]
struct MySet {
    set: BTreeSet<(i32, usize)>,
    kth: (i32, usize),
    cnt_less_or_eq: usize,
}

impl MySet {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, x: (i32, usize)) {
        if self.set.is_empty() {
            self.set.insert(x);
            self.kth = x;
            self.cnt_less_or_eq = 1;
            return;
        }

        if x <= self.kth {
            self.cnt_less_or_eq += 1;
        }
        self.set.insert(x);
    }

    fn rem(&mut self, x: (i32, usize)) {
        if self.set.len() == 1 {
            self.set.clear();
            return;
        }

        self.set.remove(&x);

        if x != self.kth {
            if x <= self.kth {
                self.cnt_less_or_eq -= 1;
            }
            return;
        }

        let next = self.get_next(self.kth);
        if !next.is_none() {
            self.kth = next.unwrap();
            return;
        }
        let prev = self.get_prev(self.kth);
        let prev = prev.unwrap();
        self.kth = prev;
        self.cnt_less_or_eq -= 1;
    }

    fn get_next(&self, x: (i32, usize)) -> Option<(i32, usize)> {
        let range = (Excluded(x), Unbounded);
        let it = self.set.range(range);
        it.min().copied()
    }

    fn get_prev(&self, x: (i32, usize)) -> Option<(i32, usize)> {
        let range = (Unbounded, Excluded(x));
        let it = self.set.range(range);
        it.max().copied()
    }

    fn get_kth(&mut self, k: usize) -> (i32, usize) {
        assert!(k < self.set.len());
        while self.cnt_less_or_eq < k + 1 {
            self.kth = self.get_next(self.kth).unwrap();
            self.cnt_less_or_eq += 1;
        }
        while self.cnt_less_or_eq > k + 1 {
            self.kth = self.get_prev(self.kth).unwrap();
            self.cnt_less_or_eq -= 1;
        }
        self.kth
    }
}

impl Solution {
    pub fn median_sliding_window(a: Vec<i32>, k: i32) -> Vec<f64> {
        let k = k as usize;
        let n = a.len();
        let mut set = MySet::new();
        let mut ans = Vec::new();
        for i in 0..n {
            set.add((a[i], i));
            if i >= k {
                set.rem((a[i - k], i - k));
            }
            if i >= k - 1 {
                let pos = (k - 1) / 2;
                let med1 = set.get_kth(pos).0;
                if k % 2 == 0 {
                    let med2 = set.get_kth(pos + 1).0;
                    let cur = (med1 as f64 + med2 as f64) * 0.5;
                    ans.push(cur);
                } else {
                    ans.push(med1 as f64);
                }
            }
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
    #[case(vec![1,3,-1,-3,5,3,6,7], 3, vec![1.00000,-1.00000,-1.00000,3.00000,5.00000,6.00000])]
    #[case(vec![1,2,3,4,2,3,1,4,2], 3, vec![2.00000,3.00000,3.00000,3.00000,2.00000,3.00000,2.00000])]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: Vec<f64>) {
        let actual = Solution::median_sliding_window(nums, k);
        assert_eq!(actual, expected);
    }
}
