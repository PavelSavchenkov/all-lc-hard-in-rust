//! Solution for https://leetcode.com/problems/palindrome-partitioning-iv
//! 1745. Palindrome Partitioning IV

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

impl Solution {
    pub fn check_partitioning(s: String) -> bool {
        let s = to_u8(&s);
        let n = s.len();

        let pal = Palindrome::new(&s);
        for l in 1..n - 1 {
            for r in l..n - 1 {
                if pal.is_palindrome(0, l)
                    && pal.is_palindrome(l, r - l + 1)
                    && pal.is_palindrome(r + 1, n - r - 1)
                {
                    return true;
                }
            }
        }
        false
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
    #[case("abcbdd", true)]
    #[case("bcbddxy", false)]
    fn case(#[case] s: String, #[case] expected: bool) {
        let actual = Solution::check_partitioning(s);
        assert_eq!(actual, expected);
    }
}
