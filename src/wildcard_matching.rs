//! Solution for https://leetcode.com/problems/wildcard-matching
//! 44. Wildcard Matching

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let s: Vec<_> = s.as_bytes().iter().cloned().collect();
        let p: Vec<_> = p.as_bytes().iter().cloned().collect();

        let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
        dp[0][0] = true;
        for i in 0..=s.len() {
            for j in 0..=p.len() {
                if !dp[i][j] {
                    continue;
                }
                if i < s.len() && j < p.len() {
                    if s[i] == p[j] || p[j] == b'?' || p[j] == b'*' {
                        dp[i + 1][j + 1] = true;
                    }
                }
                if j < p.len() && p[j] == b'*' {
                    if i + 1 <= s.len() {
                        dp[i + 1][j] = true;
                        dp[i + 1][j + 1] = true;
                    }
                    dp[i][j + 1] = true;
                }
            }
        }

        dp[s.len()][p.len()]
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("aa", "a", false)]
    #[case("aa", "*", true)]
    #[case("cb", "?a", false)]
    fn case(#[case] s: String, #[case] p: String, #[case] expected: bool) {
        let actual = Solution::is_match(s, p);
        assert_eq!(actual, expected);
    }
}
