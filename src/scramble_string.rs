//! Solution for https://leetcode.com/problems/scramble-string
//! 87. Scramble String

impl Solution {
    pub fn is_scramble(s: String, t: String) -> bool {
        let s: Vec<_> = s.as_bytes().iter().collect();
        let t: Vec<_> = t.as_bytes().iter().collect();
        let n = s.len();
        assert!(t.len() == n);

        let mut dp = vec![vec![vec![false; n]; n]; n + 1];
        for len in 1..=n {
            for i in 0..=n - len {
                for j in 0..=n - len {
                    if len == 1 {
                        dp[len][i][j] = s[i] == t[j];
                        continue;
                    }
                    let mut res = false;
                    for pref in 1..len {
                        res |= dp[pref][i][j] && dp[len - pref][i + pref][j + pref];
                        res |= dp[pref][i][j + (len - pref)] && dp[len - pref][i + pref][j];
                    }
                    dp[len][i][j] = res;
                }
            }
        }
        dp[n][0][0]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("great", "rgeat", true)]
    #[case("abcde", "caebd", false)]
    #[case("a", "a", true)]
    fn case(#[case] s1: String, #[case] s2: String, #[case] expected: bool) {
        let actual = Solution::is_scramble(s1, s2);
        assert_eq!(actual, expected);
    }
}
