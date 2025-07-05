//! Solution for https://leetcode.com/problems/palindrome-partitioning-ii
//! 132. Palindrome Partitioning II

struct Palindrome {
    odd: Vec<usize>,
    even: Vec<usize>,
    n: usize,
}

impl Palindrome {
    fn new<T: PartialEq>(s: &Vec<T>) -> Self {
        let n = s.len();
        let odd = {
            let mut p = vec![0; n];
            let mut l = 0;
            let mut r = 0;
            for i in 0..n {
                let mut len = 0;
                if r > i {
                    len = p[l + (r - i - 1)].min(r - i - 1);
                }
                while i + len + 1 < n && i >= len + 1 && s[i + len + 1] == s[i - len - 1] {
                    len += 1;
                }
                p[i] = len;
                if i + p[i] + 1 > r {
                    l = i - p[i];
                    r = i + p[i] + 1;
                }
            }
            p
        };
        let even = {
            let mut p = vec![0; n];
            let mut l = 0;
            let mut r = 0;
            for i in 0..n {
                let mut len = 0;
                if r > i {
                    len = p[l + (r - i - 1) + 1].min(r - i);
                }
                while i + len < n && i >= len + 1 && s[i + len] == s[i - len - 1] {
                    len += 1;
                }
                p[i] = len;
                if i + p[i] + 1 > r {
                    l = i - p[i];
                    r = i + p[i];
                }
            }
            p
        };
        Self { odd, even, n }
    }

    fn is_palindrome(&self, l: usize, len: usize) -> bool {
        if len % 2 == 0 {
            return self.even[l + len / 2] >= len / 2;
        }
        self.odd[l + len / 2] >= len / 2
    }
}

fn to_u8(s: &String) -> Vec<u8> {
    s.as_bytes().iter().map(|&c| c as u8).collect()
}

fn from_u8(s: &Vec<u8>) -> String {
    String::from_utf8(s.to_vec()).unwrap()
}

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let s = to_u8(&s);
        let n = s.len();

        let pal = Palindrome::new(&s);
        let mut dp = vec![usize::MAX; n + 1];
        dp[0] = 0;
        for i in 0..n {
            if dp[i] == usize::MAX {
                continue;
            }
            for len in 1..=n - i {
                if pal.is_palindrome(i, len) {
                    dp[i + len] = dp[i + len].min(dp[i] + 1);
                }
            }
        }

        (dp[n] as i32) - 1
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("aab", 1)]
    #[case("a", 0)]
    #[case("ab", 1)]
    fn case(#[case] s: String, #[case] expected: i32) {
        let actual = Solution::min_cut(s);
        assert_eq!(actual, expected);
    }
}
