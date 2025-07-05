//! Solution for https://leetcode.com/problems/arithmetic-slices-ii-subsequence
//! 446. Arithmetic Slices II - Subsequence

use std::collections::HashMap;

impl Solution {
    pub fn number_of_arithmetic_slices(a: Vec<i32>) -> i32 {
        let n = a.len();
        let a: Vec<_> = a.iter().map(|&x| x as i64).collect();

        let mut ans: u64 = 0;
        let mut dp = vec![HashMap::<i64, u64>::new(); n];
        for i in 0..n {
            for j in 0..i {
                let diff = a[i] - a[j];
                let mut prev_dp = 0;
                if let Some(val) = dp[j].get(&diff) {
                    prev_dp = *val;
                }
                let cur_dp = dp[i].entry(diff).or_insert(0);
                *cur_dp += 1 + prev_dp;
                ans += 1 + prev_dp;
            }
        }
        ans -= n as u64 * (n - 1) as u64 / 2;
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
    #[case(vec![2,4,6,8,10], 7)]
    #[case(vec![7,7,7,7,7], 16)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::number_of_arithmetic_slices(nums);
        assert_eq!(actual, expected);
    }
}
