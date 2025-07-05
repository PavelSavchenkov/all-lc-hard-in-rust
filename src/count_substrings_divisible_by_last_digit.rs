//! Solution for https://leetcode.com/problems/count-substrings-divisible-by-last-digit
//! 3448. Count Substrings Divisible By Last Digit

const D: usize = 10;

impl Solution {
    pub fn count_substrings(s: String) -> i64 {
        let s = to_u8(&s);
        let s: Vec<_> = s.iter().map(|x| (x - b'0') as usize).collect();
        let n = s.len();

        let mut dp = vec![vec![vec![0; D]; D]; n + 1];
        let mut ans = 0;
        for i in 0..n {
            for m in 1..10 {
                for rem in 0..m {
                    let cur_dp = dp[i][m][rem];
                    if cur_dp == 0 {
                        continue;
                    }
                    dp[i + 1][m][(rem * 10 + s[i]) % m] += cur_dp;
                }
                dp[i + 1][m][s[i] % m] += 1;
            }
            ans += dp[i + 1][s[i]][0];
        }
        ans
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
    #[case("12936", 11)]
    #[case("5701283", 18)]
    #[case("1010101010", 25)]
    fn case(#[case] s: String, #[case] expected: i64) {
        let actual = Solution::count_substrings(s);
        assert_eq!(actual, expected);
    }
}
