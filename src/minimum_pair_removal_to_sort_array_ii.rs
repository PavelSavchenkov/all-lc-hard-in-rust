//! Solution for https://leetcode.com/problems/minimum-pair-removal-to-sort-array-ii
//! 3510. Minimum Pair Removal to Sort Array II

use std::collections::BTreeSet;
use std::ops::Bound::{Excluded, Unbounded};

#[derive(Default)]
struct Solver {
    ids: BTreeSet<usize>,
    sums: BTreeSet<(i64, usize)>, // (a[i] + a[next(i)], i); j is next alive after i
    cnt_bad: usize,               // # a[i] > a[next(i)]
    vals: Vec<i64>,
}

impl Solver {
    fn new(a: &Vec<i32>) -> Self {
        let mut this = Self::default();
        let n = a.len();
        this.vals = vec![0; n];
        for i in 0..n {
            this.ids.insert(i);
            if i + 1 < n {
                this.sums.insert(((a[i] + a[i + 1]) as i64, i));
                if a[i] > a[i + 1] {
                    this.cnt_bad += 1;
                }
            }
            this.vals[i] = a[i] as i64;
        }
        this
    }

    fn remove_sum(&mut self, a: usize, b: usize) {
        assert!(a < b);
        let sum = self.vals[a] + self.vals[b];
        assert!(self.sums.remove(&(sum, a)));
        if self.vals[a] > self.vals[b] {
            self.cnt_bad -= 1;
        }
    }

    fn add_sum(&mut self, a: usize, b: usize) {
        assert!(a < b);
        let sum = self.vals[a] + self.vals[b];
        assert!(self.sums.insert((sum, a)));
        if self.vals[a] > self.vals[b] {
            self.cnt_bad += 1;
        }
    }

    fn step(&mut self) {
        assert!(self.cnt_bad > 0);
        let (sum, i) = self.sums.pop_first().unwrap();

        let range = (Unbounded, Excluded(i));
        let mut it = self.ids.range(range);
        let prev_id = it.next_back().copied();

        let range = (Excluded(i), Unbounded);
        let mut it = self.ids.range(range);
        let j = *it.next().unwrap();
        let next_id = it.next().copied();

        assert!(self.vals[i] + self.vals[j] == sum);
        if self.vals[i] > self.vals[j] {
            self.cnt_bad -= 1;
        }

        if let Some(ii) = prev_id {
            self.remove_sum(ii, i);
        }
        if let Some(jj) = next_id {
            self.remove_sum(j, jj);
        }

        self.vals[i] += self.vals[j];
        self.ids.remove(&j);
        if let Some(ii) = prev_id {
            self.add_sum(ii, i);
        }
        if let Some(jj) = next_id {
            self.add_sum(i, jj);
        }
    }
}

impl Solution {
    pub fn minimum_pair_removal(a: Vec<i32>) -> i32 {
        let mut solver = Solver::new(&a);

        let mut ans = 0;
        while solver.cnt_bad > 0 {
            ans += 1;
            solver.step();
        }
        ans as i32
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![5,2,3,1], 2)]
    #[case(vec![1,2,2], 0)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::minimum_pair_removal(nums);
        assert_eq!(actual, expected);
    }
}
