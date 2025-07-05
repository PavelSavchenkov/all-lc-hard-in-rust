//! Solution for https://leetcode.com/problems/constrained-subsequence-sum
//! 1425. Constrained Subsequence Sum

use std::collections::VecDeque;

impl Solution {
    pub fn constrained_subset_sum(a: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let n = a.len();

        let mut dp = a.clone();
        let mut q = VecDeque::new();
        for i in 0..n {
            if !q.is_empty() {
                dp[i] = dp[i].max(a[i] + dp[*q.front().unwrap()]);
            }
            while !q.is_empty() && dp[*q.back().unwrap()] <= dp[i] {
                q.pop_back();
            }
            q.push_back(i);
            if i >= k && *q.front().unwrap() == i - k {
                q.pop_front();
            }
        }
        let ans = *dp.iter().max().unwrap();
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
    #[case(vec![10,2,-10,5,20], 2, 37)]
    #[case(vec![-1,-2,-3], 1, -1)]
    #[case(vec![10,-2,-10,-5,20], 2, 23)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::constrained_subset_sum(nums, k);
        assert_eq!(actual, expected);
    }
}
