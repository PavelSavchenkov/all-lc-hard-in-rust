//! Solution for https://leetcode.com/problems/minimum-sum-of-values-by-dividing-array
//! 3117. Minimum Sum of Values by Dividing Array

use std::collections::VecDeque;

#[derive(Default)]
struct DequeMin {
    d: VecDeque<(i32, usize)>,
}

impl DequeMin {
    fn new() -> Self {
        Self::default()
    }

    fn push_front(&mut self, x: (i32, usize)) {
        while !self.d.is_empty() && self.d.front().unwrap().0 >= x.0 {
            self.d.pop_front();
        }
        self.d.push_front(x);
    }

    fn pop_back(&mut self, id: usize) {
        assert!(!self.d.is_empty());
        if self.d.back().unwrap().1 == id {
            self.d.pop_back();
        }
    }

    fn get_min(&self) -> i32 {
        self.d.back().unwrap().0
    }
}

fn bit(mask: i32, b: usize) -> bool {
    ((mask >> b) & 1) == 1
}

const B: usize = 18;

impl Solution {
    pub fn minimum_value_sum(a: Vec<i32>, and_values: Vec<i32>) -> i32 {
        let n = a.len();
        let m = and_values.len();

        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;
        for &val in &and_values {
            let mut ndp = vec![i32::MAX; n + 1];
            let mut deque = DequeMin::new();
            let mut R = 0;
            let mut L = 0;

            let mut last = vec![-1; B];
            for i in 0..=n {
                let mut r = n as i32;
                let mut l = -1;
                for b in 0..B {
                    if !bit(val, b) {
                        r = r.min(last[b]);
                    } else {
                        l = l.max(last[b]);
                    }
                }
                l += 1;
                r += 1;
                let l = l as usize;
                let r = r as usize;
                // [l, r)
                if l < r {
                    assert!(i > 0);
                    assert!(R <= r);
                    while R < r {
                        deque.push_front((dp[R], R));
                        R += 1;
                    }
                    assert!(L <= l);
                    while L < l {
                        deque.pop_back(L);
                        L += 1;
                    }
                    let min_prev_dp = deque.get_min();
                    if min_prev_dp < i32::MAX {
                        ndp[i] = min_prev_dp + a[i - 1];
                    }
                }
                if i < n {
                    for b in 0..B {
                        if !bit(a[i], b) {
                            last[b] = i as i32;
                        }
                    }
                }
            }

            dp = ndp;
        }
        let ans = dp[n];
        if ans == i32::MAX {
            return -1;
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
    #[case(vec![1,4,3,3,2], vec![0,3,3,2], 12)]
    #[case(vec![2,3,5,7,7,7,5], vec![0,7,5], 17)]
    #[case(vec![1,2,3,4], vec![2], -1)]
    fn case(#[case] nums: Vec<i32>, #[case] and_values: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::minimum_value_sum(nums, and_values);
        assert_eq!(actual, expected);
    }
}
