//! Solution for https://leetcode.com/problems/minimum-changes-to-make-k-semi-palindromes
//! 2911. Minimum Changes to Make K Semi-palindromes

fn remin(a: &mut usize, b: usize) {
    if *a > b {
        *a = b;
    }
}

impl Solution {
    pub fn minimum_changes(s: String, k: i32) -> i32 {
        let s = to_u8(&s);
        let n = s.len();
        let k = k as usize;

        let mut min_change = vec![vec![vec![usize::MAX; n + 1]; n]; n];
        for len in 2..=n {
            for l in 0..=n - len {
                let r = l + len - 1;
                for d in 1..=len {
                    if len % d != 0 {
                        continue;
                    }
                    let mut change = 0;
                    if d == 1 {
                        if s[l] != s[r] {
                            change += 1;
                        }
                        if len > 3 {
                            change += min_change[l + 1][r - 1][d];
                        }
                    } else if d < len {
                        assert!(d <= len / 2);
                        for i in 0..d {
                            if s[l + i] != s[r - d + 1 + i] {
                                change += 1;
                            }
                        }
                        if len / d > 2 {
                            change += min_change[l + d][r - d][d];
                        }
                    }
                    min_change[l][r][d] = change;
                }
            }
        }

        let mut min_change_distilled = vec![vec![usize::MAX; n]; n];
        for l in 0..n {
            for r in l + 1..n {
                let mut mn = usize::MAX;
                for d in 1..r - l + 1 {
                    remin(&mut mn, min_change[l][r][d]);
                }
                min_change_distilled[l][r] = mn;
            }
        }

        let mut dp = vec![vec![usize::MAX; k + 1]; n + 1];
        dp[0][0] = 0;
        for i in 0..n {
            for taken in 0..k {
                let cur_dp = dp[i][taken];
                if cur_dp == usize::MAX {
                    continue;
                }
                for j in i + 1..n {
                    remin(
                        &mut dp[j + 1][taken + 1],
                        cur_dp + min_change_distilled[i][j],
                    );
                }
            }
        }

        dp[n][k] as i32
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
    #[case("abcac", 2, 1)]
    #[case("abcdef", 2, 2)]
    #[case("aabbaa", 3, 0)]
    #[case("aaabacacbb", 1, 3)]
    fn case(#[case] s: String, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::minimum_changes(s, k);
        assert_eq!(actual, expected);
    }
}
