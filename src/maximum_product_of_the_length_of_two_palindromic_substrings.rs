//! Solution for https://leetcode.com/problems/maximum-product-of-the-length-of-two-palindromic-substrings
//! 1960. Maximum Product of the Length of Two Palindromic Substrings

fn calc_starts_from(s: &Vec<u8>) -> Vec<usize> {
    let n = s.len();
    let pal = Palindrome::new(&s);
    let mut ans = vec![0; n];
    for i in 0..n {
        let l = i - pal.odd[i];
        ans[l] = ans[l].max((i - l) * 2 + 1);
    }
    for i in 1..n {
        if ans[i - 1] >= 2 {
            ans[i] = ans[i].max(ans[i - 1] - 2);
        }
    }
    for i in (0..n - 1).rev() {
        ans[i] = ans[i].max(ans[i + 1]);
    }
    ans
}

impl Solution {
    pub fn max_product(s: String) -> i64 {
        let mut s = to_u8(&s);

        let suff = calc_starts_from(&s);

        s.reverse();
        let mut pref = calc_starts_from(&s);
        s.reverse();
        pref.reverse();

        let mut ans = 0;
        for i in 1..s.len() {
            let left = pref[i - 1];
            let right = suff[i];
            ans = ans.max(left as i64 * right as i64);
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

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("ababbb", 9)]
    #[case("zaaaxbbby", 9)]
    fn case(#[case] s: String, #[case] expected: i64) {
        let actual = Solution::max_product(s);
        assert_eq!(actual, expected);
    }
}
