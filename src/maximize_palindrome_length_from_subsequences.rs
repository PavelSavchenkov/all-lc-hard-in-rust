//! Solution for https://leetcode.com/problems/maximize-palindrome-length-from-subsequences
//! 1771. Maximize Palindrome Length From Subsequences

const A: usize = 26;

impl Solution {
    pub fn longest_palindrome(word1: String, word2: String) -> i32 {
        let word1 = to_u8(&word1);
        let word2 = to_u8(&word2);

        let mut s = word1.clone();
        s.extend(word2.iter());

        let n = s.len();
        let mut next = vec![vec![n; A]; n];
        for i in (0..n).rev() {
            if i + 1 < n {
                for c in 0..A {
                    next[i][c] = next[i + 1][c];
                }
            }
            let c = (s[i] - b'a') as usize;
            next[i][c] = i;
        }

        let mut prev = vec![vec![0; A]; n];
        for i in 0..n {
            if i > 0 {
                for c in 0..A {
                    prev[i][c] = prev[i - 1][c];
                }
            }
            let c = (s[i] - b'a') as usize;
            prev[i][c] = i;
        }

        let mut ans = 0;
        let mut dp = vec![vec![0; n]; n];
        for len in 1..=n {
            for l in 0..=n - len {
                let r = l + len - 1;
                let mut ndp = 0;
                for c in 0..A {
                    let ll = next[l][c];
                    let rr = prev[r][c];
                    if ll > rr {
                        continue;
                    }
                    let mut cand = 0;
                    if ll == rr {
                        cand = cand.max(1);
                    } else if ll + 1 == r {
                        cand = cand.max(2);
                    } else {
                        cand = cand.max(2 + dp[ll + 1][rr - 1]);
                    }
                    if ll < word1.len() && word1.len() <= rr {
                        ans = ans.max(cand);
                    }
                    ndp = ndp.max(cand);
                }
                dp[l][r] = ndp;
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
    #[case("cacb", "cbba", 5)]
    #[case("ab", "ab", 3)]
    #[case("aa", "bb", 0)]
    fn case(#[case] word1: String, #[case] word2: String, #[case] expected: i32) {
        let actual = Solution::longest_palindrome(word1, word2);
        assert_eq!(actual, expected);
    }
}
