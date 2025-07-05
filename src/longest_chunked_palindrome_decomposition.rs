//! Solution for https://leetcode.com/problems/longest-chunked-palindrome-decomposition
//! 1147. Longest Chunked Palindrome Decomposition

impl Solution {
    pub fn longest_decomposition(s: String) -> i32 {
        let s = to_u8(&s);
        let n = s.len();

        let mut lcp = vec![vec![0; n]; n];
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if s[i] == s[j] {
                    lcp[i][j] = 1;
                    if i + 1 < n && j + 1 < n {
                        lcp[i][j] += lcp[i + 1][j + 1];
                    }
                }
            }
        }

        let mut dp = vec![i32::MIN; n + 1];
        for len in 0..=n {
            if len % 2 != n % 2 {
                continue;
            }
            if len == 0 {
                dp[len] = 0;
                continue;
            }
            dp[len] = 1;
            let l = (n - len) / 2;
            let r = l + len - 1;
            for pref in 1..=len / 2 {
                if lcp[l][r - pref + 1] >= pref {
                    dp[len] = dp[len].max(dp[len - pref * 2] + 2);
                }
            }
        }
        dp[n]
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
    #[case("ghiabcdefhelloadamhelloabcdefghi", 7)]
    #[case("merchant", 1)]
    #[case("antaprezatepzapreanta", 11)]
    fn case(#[case] text: String, #[case] expected: i32) {
        let actual = Solution::longest_decomposition(text);
        assert_eq!(actual, expected);
    }
}
