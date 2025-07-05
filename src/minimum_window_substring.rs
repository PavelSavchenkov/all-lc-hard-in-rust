//! Solution for https://leetcode.com/problems/minimum-window-substring
//! 76. Minimum Window Substring

const A: usize = 26 + 26;

fn char_code(c: u8) -> usize {
    if b'a' <= c && c <= b'z' {
        return (c - b'a') as usize;
    }
    assert!(b'A' <= c && c <= b'Z');
    (b'z' - b'a' + c - b'A') as usize
}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s = s.as_bytes();
        let t = t.as_bytes();

        let update_char =
            |c: u8, need_cnt: &[i32], have_cnt: &mut [i32], cnt_same: &mut usize, is_add| {
                let c = char_code(c);
                if need_cnt[c] <= have_cnt[c] {
                    *cnt_same -= 1;
                }
                have_cnt[c] += if is_add { 1 } else { -1 };
                if need_cnt[c] <= have_cnt[c] {
                    *cnt_same += 1;
                }
            };

        let mut need_cnt = [0 as i32; A];
        for &b in t {
            need_cnt[char_code(b)] += 1;
        }

        let mut have_cnt = [0 as i32; A];
        let mut cnt_same = 0;
        for c in 0..A {
            if need_cnt[c] == have_cnt[c] {
                cnt_same += 1;
            }
        }

        let mut l = 0;
        let mut r = 0;
        let mut j = 0;
        for i in 0..s.len() {
            while j < s.len() && cnt_same < A {
                update_char(s[j], &need_cnt, &mut have_cnt, &mut cnt_same, true);
                j += 1;
            }
            if cnt_same < A {
                break;
            }
            if j - i < r - l || r == 0 {
                l = i;
                r = j;
            }
            update_char(s[i], &need_cnt, &mut have_cnt, &mut cnt_same, false);
        }

        if r == 0 {
            return "".to_string();
        }

        String::from_utf8(s[l..r].to_vec()).unwrap()
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("ADOBECODEBANC", "ABC", "BANC")]
    #[case("a", "a", "a")]
    #[case("a", "aa", "")]
    fn case(#[case] s: String, #[case] t: String, #[case] expected: String) {
        let actual = Solution::min_window(s, t);
        assert_eq!(actual, expected);
    }
}
