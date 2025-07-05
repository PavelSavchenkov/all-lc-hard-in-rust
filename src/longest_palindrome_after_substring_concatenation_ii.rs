//! Solution for https://leetcode.com/problems/longest-palindrome-after-substring-concatenation-ii
//! 3504. Longest Palindrome After Substring Concatenation II

impl Solution {
    pub fn longest_palindrome(s: String, t: String) -> i32 {
        let mut s = to_u8(&s);
        let mut t = to_u8(&t);

        let mut ans = 1;

        for it in 0..2 {
            let n = s.len();
            let m = t.len();

            let mut s_rev = s.clone();
            s_rev.reverse();

            let mut lcp = vec![vec![0; m + 1]; n + 1];
            for i in (0..n).rev() {
                for j in (0..m).rev() {
                    if s_rev[i] == t[j] {
                        lcp[i][j] = 1 + lcp[i + 1][j + 1];
                    }
                }
            }

            for j in 0..m {
                for is_odd in 0..2 {
                    if j == 0 && is_odd == 1 {
                        continue;
                    }
                    let mut len = 0;
                    let shift = 1 + is_odd;
                    if j >= shift {
                        let jj = j - shift;
                        while j + len < m && jj >= len && t[j + len] == t[jj - len] {
                            len += 1;
                        }
                    }
                    let from = j + len;
                    let mut mx_from = 0;
                    for i in 0..n {
                        mx_from = mx_from.max(lcp[i][from]);
                    }
                    let cur = len * 2 + is_odd + mx_from * 2;
                    ans = ans.max(cur);
                }
            }

            std::mem::swap(&mut s, &mut t);
            s.reverse();
            t.reverse();
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
    #[case("a", "a", 2)]
    #[case("abc", "def", 1)]
    #[case("b", "aaaa", 4)]
    #[case("abcde", "ecdba", 5)]
    fn case(#[case] s: String, #[case] t: String, #[case] expected: i32) {
        let actual = Solution::longest_palindrome(s, t);
        assert_eq!(actual, expected);
    }
}
