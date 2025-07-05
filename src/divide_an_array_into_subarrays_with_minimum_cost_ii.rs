//! Solution for https://leetcode.com/problems/divide-an-array-into-subarrays-with-minimum-cost-ii
//! 3013. Divide an Array Into Subarrays With Minimum Cost II

use std::collections::BTreeSet;

#[derive(Default)]
struct Set {
    topk: BTreeSet<(i32, usize)>,
    sum_topk: i64,
    k: usize,
    rest: BTreeSet<(i32, usize)>,
}

impl Set {
    fn new(k: usize) -> Self {
        Self {
            k,
            ..Self::default()
        }
    }

    fn add(&mut self, x: (i32, usize)) {
        self.add_topk(&x);
        self.norm();
    }

    fn del(&mut self, x: (i32, usize)) {
        self.rest.remove(&x);
        self.remove_topk(&x);
        self.norm();
    }

    fn remove_topk(&mut self, x: &(i32, usize)) {
        if self.topk.remove(x) {
            self.sum_topk -= x.0 as i64;
        }
    }

    fn add_topk(&mut self, x: &(i32, usize)) {
        self.sum_topk += x.0 as i64;
        self.topk.insert(*x);
    }

    fn norm(&mut self) {
        while !self.topk.is_empty() && !self.rest.is_empty() {
            if self.topk.last().unwrap().0 > self.rest.first().unwrap().0 {
                let last = *self.topk.last().unwrap();
                self.remove_topk(&last);
                self.rest.insert(last);
            } else {
                break;
            }
        }

        while self.topk.len() > self.k {
            let last = *self.topk.last().unwrap();
            self.remove_topk(&last);
            self.rest.insert(last);
        }

        while self.topk.len() < self.k && !self.rest.is_empty() {
            let first = self.rest.pop_first().unwrap();
            self.add_topk(&first);
        }
    }
}

impl Solution {
    pub fn minimum_cost(a: Vec<i32>, k: i32, dist: i32) -> i64 {
        let k = k as usize;
        let dist = dist as usize;
        let n = a.len();

        let mut ans = i64::MAX;
        let mut set = Set::new(k - 2);
        let mut l = 1;
        let mut r = 1;
        for i1 in 1..n {
            let R = (i1 + dist + 1).min(n);
            while r < R {
                set.add((a[r], r));
                r += 1;
            }
            while l <= i1 {
                set.del((a[l], l));
                l += 1;
            }
            if set.topk.len() == k - 2 {
                let cur = set.sum_topk + a[i1] as i64 + a[0] as i64;
                ans = ans.min(cur);
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
    #[case(vec![1,3,2,6,4,2], 3, 3, 5)]
    #[case(vec![10,1,2,2,2,1], 4, 3, 15)]
    #[case(vec![10,8,18,9], 3, 1, 36)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] dist: i32, #[case] expected: i64) {
        let actual = Solution::minimum_cost(nums, k, dist);
        assert_eq!(actual, expected);
    }
}
