//! Solution for https://leetcode.com/problems/shortest-palindrome
//! 214. Shortest Palindrome

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
    pub fn shortest_palindrome(s: String) -> String {
        if s.is_empty() {
            return s;
        }

        let s: Vec<_> = s.as_bytes().iter().cloned().collect();
        let n = s.len();

        let pal = Palindrome::new(&s);
        let mut r = 0;
        for i in 0..n {
            if pal.is_palindrome(0, i + 1) {
                r = i;
            }
        }

        let suff = s[r + 1..s.len()].to_vec();
        let mut pref = suff.clone();
        pref.reverse();
        pref.extend(s);

        String::from_utf8(pref.to_vec()).unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("aacecaaa", "aaacecaaa")]
    #[case("abcd", "dcbabcd")]
    fn case(#[case] s: String, #[case] expected: String) {
        let actual = Solution::shortest_palindrome(s);
        assert_eq!(actual, expected);
    }
}
