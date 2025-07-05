//! Solution for https://leetcode.com/problems/minimum-operations-to-make-elements-within-k-subarrays-equal
//! 3505. Minimum Operations to Make Elements Within K Subarrays Equal

use std::collections::BTreeSet;

#[derive(Default)]
struct Set {
    upper: BTreeSet<(i32, usize)>,
    upper_sum: i64,
    lower: BTreeSet<(i32, usize)>,
    lower_sum: i64,
}

impl Set {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, item: (i32, usize)) {
        if self.lower.is_empty() || *self.lower.last().unwrap() <= item {
            self.upper.insert(item);
            self.upper_sum += item.0 as i64;
        } else {
            self.lower.insert(item);
            self.lower_sum += item.0 as i64;
        }
        self.norm();
    }

    fn remove(&mut self, item: (i32, usize)) {
        if self.lower.remove(&item) {
            self.lower_sum -= item.0 as i64;
        }
        if self.upper.remove(&item) {
            self.upper_sum -= item.0 as i64;
        }
        self.norm();
    }

    fn norm(&mut self) {
        while self.upper.len() > self.lower.len() {
            let item = self.upper.pop_first().unwrap();
            self.upper_sum -= item.0 as i64;
            self.lower.insert(item);
            self.lower_sum += item.0 as i64;
        }
        while self.lower.len() > self.upper.len() + 1 {
            let item = self.lower.pop_last().unwrap();
            self.lower_sum -= item.0 as i64;
            self.upper.insert(item);
            self.upper_sum += item.0 as i64;
        }
    }

    fn cost_move_to_median(&self) -> i64 {
        assert!(!self.lower.is_empty());
        assert!(self.lower.len() >= self.upper.len());
        let med = self.lower.last().unwrap().0 as i64;
        let mut ans = med * self.lower.len() as i64 - self.lower_sum;
        ans += self.upper_sum - med * self.upper.len() as i64;
        ans
    }
}

impl Solution {
    pub fn min_operations(a: Vec<i32>, x: i32, k: i32) -> i64 {
        let x = x as usize;
        let k = k as usize;
        let n = a.len();

        let mut cost = vec![i64::MAX; n];
        let mut set = Set::new();
        for i in 0..n {
            set.add((a[i], i));
            if i >= x {
                set.remove((a[i - x], i - x));
            }
            if i + 1 >= x {
                cost[i] = set.cost_move_to_median();
            }
        }

        let mut dp = vec![0; n + 1];
        for it in 0..k {
            let mut ndp = vec![i64::MAX; n + 1];
            for i in 0..n {
                if i > 0 {
                    ndp[i + 1] = ndp[i];
                }
                if i + 1 >= x && dp[i + 1 - x] < i64::MAX {
                    ndp[i + 1] = ndp[i + 1].min(dp[i + 1 - x] + cost[i]);
                }
            }
            dp = ndp;
        }
        dp[n]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![5,-2,1,3,7,3,6,4,-1], 3, 2, 8)]
    #[case(vec![9,-2,-2,-2,1,5], 2, 2, 3)]
    fn case(#[case] nums: Vec<i32>, #[case] x: i32, #[case] k: i32, #[case] expected: i64) {
        let actual = Solution::min_operations(nums, x, k);
        assert_eq!(actual, expected);
    }
}
