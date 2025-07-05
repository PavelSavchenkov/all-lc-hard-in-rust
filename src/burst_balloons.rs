//! Solution for https://leetcode.com/problems/burst-balloons
//! 312. Burst Balloons

impl Solution {
    pub fn max_coins(a: Vec<i32>) -> i32 {
        let n = a.len();

        let mut dp = vec![vec![0; n]; n];
        for len in 1..=n {
            for l in 0..=n - len {
                let r = l + len - 1;
                let L = if l == 0 { 1 } else { a[l - 1] };
                let R = if r == n - 1 { 1 } else { a[r + 1] };
                for i in l..=r {
                    let mut cur = L * a[i] * R;
                    if i > l {
                        cur += dp[l][i - 1];
                    }
                    if i < r {
                        cur += dp[i + 1][r];
                    }
                    dp[l][r] = dp[l][r].max(cur);
                }
            }
        }

        dp[0][n - 1]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(vec![3,1,5,8], 167)]
    #[case(vec![1,5], 10)]
    fn case(#[case] nums: Vec<i32>, #[case] expected: i32) {
        let actual = Solution::max_coins(nums);
        assert_eq!(actual, expected);
    }
}
