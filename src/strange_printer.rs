//! Solution for https://leetcode.com/problems/strange-printer
//! 664. Strange Printer

const A: usize = 26;

impl Solution {
    pub fn strange_printer(s: String) -> i32 {
        let s = to_u8(&s);
        let n = s.len();

        let mut dp = vec![vec![vec![u32::MAX; A]; n]; n];
        for len in 1..=n {
            for l in 0..=n - len {
                let r = l + len - 1;
                let mut min_dp = u32::MAX;
                for c in 0..A {
                    let mut ndp = u32::MAX;
                    if len == 1 {
                        if s[l] == b'a' + c as u8 {
                            ndp = 0;
                        } else {
                            ndp = 1;
                        }
                    } else {
                        for i in l..=r {
                            if s[i] == b'a' + c as u8 {
                                let mut cur = 0;
                                if i < r {
                                    cur += dp[i + 1][r][c];
                                }
                                if l < i {
                                    cur += dp[l][i - 1][c];
                                }
                                ndp = ndp.min(cur);
                            }
                        }
                    }
                    dp[l][r][c] = ndp;
                    min_dp = min_dp.min(ndp);
                }

                for c in 0..A {
                    dp[l][r][c] = dp[l][r][c].min(min_dp + 1);
                }
            }
        }

        let mut ans = u32::MAX;
        for c in 0..A {
            ans = ans.min(dp[0][n - 1][c] + 1);
        }
        ans as i32
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
    #[case("aaabbb", 2)]
    #[case("aba", 2)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::strange_printer(s);
        assert_eq!(actual, expected);
    }
}
