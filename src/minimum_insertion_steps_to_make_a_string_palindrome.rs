//! Solution for https://leetcode.com/problems/minimum-insertion-steps-to-make-a-string-palindrome
//! 1312. Minimum Insertion Steps to Make a String Palindrome

impl Solution {
    pub fn min_insertions(s: String) -> i32 {
        let s = to_u8(&s);
        let n = s.len();

        let mut dp = vec![vec![usize::MAX; n]; n];
        for len in 1..=n {
            for l in 0..=n - len {
                let r = l + len - 1;
                let mut ndp;
                match len {
                    1 => {
                        ndp = 0;
                    }
                    2 => {
                        if s[l] == s[r] {
                            ndp = 0;
                        } else {
                            ndp = 1;
                        }
                    }
                    _ => {
                        ndp = (dp[l + 1][r] + 1).min(dp[l][r - 1] + 1);
                        if s[l] == s[r] {
                            ndp = ndp.min(dp[l + 1][r - 1]);
                        }
                    }
                }
                dp[l][r] = ndp;
            }
        }
        dp[0][n - 1] as i32
    }
}

fn to_u8(s: &String) -> Vec<u8> {
    s.as_bytes().iter().map(|&c| c as u8).collect()
}

fn to_u8_vec(s: &Vec<String>) -> Vec<Vec<u8>> {
    s.iter().map(|ss| to_u8(&ss)).collect()
}

fn from_u8(s: &Vec<u8>) -> String {
    String::from_utf8(s.to_vec()).unwrap()
}

fn from_u8_vec(s: &Vec<Vec<u8>>) -> Vec<String> {
    s.iter().map(|ss| from_u8(&ss)).collect()
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("zzazz", 0)]
    #[case("mbadm", 2)]
    #[case("leetcode", 5)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::min_insertions(s);
        assert_eq!(actual, expected);
    }
}
