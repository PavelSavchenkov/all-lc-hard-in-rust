//! Solution for https://leetcode.com/problems/regular-expression-matching
//! 10. Regular Expression Matching

fn covers(p: u8, s: u8) -> bool {
    p == s || p == b'.'
}

impl Solution {
    pub fn is_match(ss: String, pp: String) -> bool {
        let s = ss.as_bytes();
        let p = pp.as_bytes();

        // eprintln!("s={ss:#?}, p={pp:#?}");

        let n = s.len();
        let m = p.len();

        let mut dp = vec![vec![false; m + 1]; n + 1];
        dp[0][0] = true;
        for i in 0..=n {
            for j in 0..m {
                if !dp[i][j] || p[j] == b'*' {
                    continue;
                }
                // eprintln!("dp[{i}][{j}] is true");
                if j + 1 < m && p[j + 1] == b'*' {
                    if i < n && covers(p[j], s[i]) {
                        dp[i + 1][j] = true;
                    }
                    dp[i][j + 2] = true;
                } else {
                    if i < n && covers(p[j], s[i]) {
                        dp[i + 1][j + 1] = true;
                    }
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
    #[case("aa", "a", false)]
    #[case("aa", "a*", true)]
    #[case("ab", ".*", true)]
    fn case(#[case] s: String, #[case] p: String, #[case] expected: bool) {
        let actual = Solution::is_match(s, p);
        assert_eq!(actual, expected);
    }
}
