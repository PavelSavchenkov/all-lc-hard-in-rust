//! Solution for https://leetcode.com/problems/count-complete-substrings
//! 2953. Count Complete Substrings

const A: usize = 26;

impl Solution {
    pub fn count_complete_substrings(word: String, k: i32) -> i32 {
        let s = to_u8(&word);
        let k = k as usize;
        let n = s.len();

        let update_cnt = |cnt: &mut Vec<i32>, cnt_ok: &mut usize, ch: u8, change: i32| {
            let c = (ch - b'a') as usize;
            if cnt[c] == k as i32 {
                *cnt_ok -= 1;
            }
            cnt[c] += change;
            if cnt[c] == k as i32 {
                *cnt_ok += 1;
            }
        };

        let mut ans = 0;
        for chars in 1..=A {
            let len = k * chars;
            if len > n {
                break;
            }

            let mut last_bad = -1;
            let mut cnt_ok = 0;
            let mut cnt = vec![0; A];
            for r in 0..n {
                update_cnt(&mut cnt, &mut cnt_ok, s[r], 1);
                if r >= len - 1 {
                    if r >= len {
                        update_cnt(&mut cnt, &mut cnt_ok, s[r - len], -1);
                    }
                    if cnt_ok == chars && (r + 1 - len) as i32 > last_bad {
                        ans += 1;
                    }
                }
                if r + 1 < n && (s[r] as i32 - s[r + 1] as i32).abs() > 2 {
                    last_bad = r as i32;
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
    #[case("igigee", 2, 3)]
    #[case("aaabbbccc", 3, 6)]
    #[case("az", 1, 2)]
    fn case(#[case] word: String, #[case] k: i32, #[case] expected: i32) {
        let actual = Solution::count_complete_substrings(word, k);
        assert_eq!(actual, expected);
    }
}
