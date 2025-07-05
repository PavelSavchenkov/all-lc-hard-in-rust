//! Solution for https://leetcode.com/problems/distinct-subsequences
//! 115. Distinct Subsequences

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let n = s.len();
        let m = t.len();

        let mut dp = vec![vec![0; m + 1]; n + 1];
        dp[0][0] = 1;
        for j in 0..=m {
            for i in 0..=n {
                if i > 0 {
                    dp[i][j] += dp[i - 1][j];
                }
                if i > 0 && j > 0 && s[i - 1] == t[j - 1] {
                    dp[i][j] += dp[i - 1][j - 1];
                }
            }
        }

        dp[n][m]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("rabbbit", "rabbit", 3)]
    #[case("babgbag", "bag", 5)]
    fn case(#[case] s: String, #[case] t: String, #[case] expected: i32) {
        let actual = Solution::num_distinct(s, t);
        assert_eq!(actual, expected);
    }
}
