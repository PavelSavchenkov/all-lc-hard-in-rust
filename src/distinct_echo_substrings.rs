//! Solution for https://leetcode.com/problems/distinct-echo-substrings
//! 1316. Distinct Echo Substrings

impl Solution {
    pub fn distinct_echo_substrings(s: String) -> i32 {
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

        let mut max_len_earlier = vec![0; n];
        for i in 0..n {
            for j in 0..i {
                let len = lcp[j][i];
                max_len_earlier[i] = max_len_earlier[i].max(len);
            }
        }

        let mut ans = 0;
        for len in 1..=n / 2 {
            for i in 0..=n - 2 * len {
                if max_len_earlier[i] < len * 2 {
                    if lcp[i][i + len] >= len {
                        ans += 1;
                    }
                }
            }
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
    #[case("abcabcabc", 3)]
    #[case("leetcodeleetcode", 2)]
    fn case(#[case] text: String, #[case] expected: i32) {
        let actual = Solution::distinct_echo_substrings(text);
        assert_eq!(actual, expected);
    }
}
