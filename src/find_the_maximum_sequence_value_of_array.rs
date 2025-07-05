//! Solution for https://leetcode.com/problems/find-the-maximum-sequence-value-of-array
//! 3287. Find the Maximum Sequence Value of Array

const B: usize = 7;

fn calc_pref_dp(a: &Vec<i32>, k: usize) -> Vec<Vec<bool>> {
    let n = a.len();
    let mut dp = vec![vec![false; 1 << B]; n + 1];
    dp[0][0] = true;
    let mut ans_dp = vec![vec![false; 1 << B]; n + 1];
    for i in 0..n {
        for cnt in (0..=(k - 1).min(i)).rev() {
            for or in 0..1 << B {
                if dp[cnt][or] {
                    dp[cnt + 1][or | (a[i] as usize)] = true;
                }
            }
        }
        for or in 0..1 << B {
            if dp[k][or] {
                ans_dp[i + 1][or] = true;
            }
        }
    }
    ans_dp
}

impl Solution {
    pub fn max_value(mut a: Vec<i32>, k: i32) -> i32 {
        let n = a.len();
        let k = k as usize;

        let pref_dp = calc_pref_dp(&a, k);

        a.reverse();
        let suff_dp = calc_pref_dp(&a, k);
        a.reverse();

        let mut ans = 0;
        for pref in k..=n - k {
            let suff = n - pref;
            for left in 0..1 << B {
                for right in 0..1 << B {
                    if pref_dp[pref][left] && suff_dp[suff][right] {
                        ans = ans.max(left ^ right);
                    }
                }
            }
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
    #[case(vec![2,6,7], 1, 5)]
    #[case(vec![4,2,5,6,7], 2, 2)]
    fn case(#[case] nums: Vec<i32>, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::max_value(nums, k);
        assert_eq!(actual, expected);
    }
}
