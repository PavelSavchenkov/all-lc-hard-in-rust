//! Solution for https://leetcode.com/problems/minimum-operations-to-make-a-subsequence
//! 1713. Minimum Operations to Make a Subsequence

use std::collections::HashMap;

impl Solution {
    pub fn min_operations(target: Vec<i32>, arr: Vec<i32>) -> i32 {
        let mut pos_target = HashMap::new();
        for i in 0..target.len() {
            pos_target.insert(target[i], i);
        }

        let mut a = Vec::new();
        for &x in &arr {
            if let Some(&pos) = pos_target.get(&x) {
                a.push(pos as i32);
            }
        }
        drop(arr);

        // find LIS
        let mut dp = vec![i32::MAX; a.len() + 1];
        dp[0] = i32::MIN;
        let mut LIS = 0;
        for &x in &a {
            let i = dp.partition_point(|&d| d < x);
            dp[i] = x;
            LIS = LIS.max(i);
        }

        let ans = target.len() - LIS;
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
    #[case(vec![5,1,3], vec![9,4,2,3,4], 2)]
    #[case(vec![6,4,8,1,3,2], vec![4,7,6,2,3,8,6,1], 3)]
    fn case(#[case] target: Vec<i32>, #[case] arr: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::min_operations(target, arr);
        assert_eq!(actual, expected);
    }
}
